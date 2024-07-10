use bevy::prelude::*;
use super::{components::*, player::components::{PlayerLoot, ShipLayout}, ship_blocks::components::Block};
use crate::{game::enemies::components::Enemy, general::components::SceneElement};

pub fn update_destructibles(
    destructibles: Query<(Entity, &Destructible, Option<&Enemy>, Option<&Block>)>,
    mut enemy_death_ev: EventWriter<EnemyDeathEvent>,
    mut block_destruction_ev: EventWriter<BlockDestructionEvent>
) {
    for (destruct_entity, destruct_stats, enemy, block) in destructibles.iter() {
        if destruct_stats.hp <= 0 {
            if enemy.is_some() {
                enemy_death_ev.send(EnemyDeathEvent(destruct_entity));
            }
            if block.is_some() {
                block_destruction_ev.send(BlockDestructionEvent(destruct_entity));
            }
        }
    }
}

pub fn animate_loot(
    mut loot_query: Query<(&mut Transform, &mut Loot)>,
    time: Res<Time>
) {
    for (mut loot_transform, mut loot) in loot_query.iter_mut() {
        loot_transform.translation += loot.dir * 2.0 * loot.speed_mult * time.delta_seconds();
        loot_transform.rotate_z(loot.speed_mult * time.delta_seconds());
        if loot.speed_mult > 0.0 {
            loot.speed_mult -= time.delta_seconds();
        }
    }
}

pub fn trigger_animation(
    mut ev_damaged: EventReader<DamagedEvent>,
    mut target_query: Query<&mut Sprite, With<Destructible>>,
) {
    for ev in ev_damaged.read() {
        if let Ok(mut sprite) = target_query.get_mut(ev.0) {
            sprite.color = Color::RED;
        }
    }
}

pub fn damaged_animation(
    mut target_query: Query<(&mut Sprite, &mut Destructible)>,
    time: Res<Time>,
) {
    for (mut sprite, mut destructible) in target_query.iter_mut() {
        if sprite.color == Color::RED {
            destructible.time_spent_red += time.delta_seconds();
            if destructible.time_spent_red >= 0.15 {
                sprite.color = Color::WHITE;
                destructible.time_spent_red = 0.0;
            }
        }
    }
}

pub fn collision_detection(
    mut collider_query: Query<(&GlobalTransform, Entity, &Collider)>,
    mut collision_ev: EventWriter<CollisionEvent>,
) {
    let mut combinations = collider_query.iter_combinations_mut();
    while let Some([(glob_transform_1, entity_1, collider_1), 
                    (glob_transform_2, entity_2, collider_2)]) = combinations.fetch_next() {
        if glob_transform_1.translation().distance(glob_transform_2.translation()) < collider_1.radius + collider_2.radius {
            collision_ev.send(CollisionEvent(entity_1, entity_2));
        }
    }
}

pub fn collision_physics_logic(
    mut collider_query: Query<(&mut Transform, &GlobalTransform, &Collider)>,
    mut collision_ev: EventReader<CollisionEvent>,
    time: Res<Time>
) {
    for ev in collision_ev.read() {
        if let Ok(
            [(mut transform_1, glob_transform_1, collider_1),
            (mut transform_2, glob_transform_2, collider_2)] 
        ) = collider_query.get_many_mut([ev.0, ev.1]) {
            if collider_1.collision_response == CollisionResponse::Moves && collider_2.collision_response == CollisionResponse::Moves {
                let dir_away = transform_1.translation - transform_2.translation;
                // how_close: 0 means colliders baraly touch and 1 means the two objects are in each other
                let how_close = 1.0 - dir_away.length() / (collider_1.radius + collider_2.radius);
                transform_1.translation += dir_away.normalize() * how_close * 100.0 * time.delta_seconds();
                transform_2.translation -= dir_away.normalize() * how_close * 100.0 * time.delta_seconds();
            } else 
            if collider_1.collision_response == CollisionResponse::Stays && collider_2.collision_response == CollisionResponse::Moves {
                let dir_away = glob_transform_1.translation() - transform_2.translation;
                let how_close = 1.0 - dir_away.length() / (collider_1.radius + collider_2.radius);
                transform_2.translation -= dir_away.normalize() * how_close * 500.0 * time.delta_seconds();
            } else
            if collider_2.collision_response == CollisionResponse::Stays && collider_1.collision_response == CollisionResponse::Moves {
                let dir_away = transform_1.translation - glob_transform_2.translation();
                let how_close = 1.0 - dir_away.length() / (collider_1.radius + collider_2.radius);
                transform_1.translation += dir_away.normalize() * how_close * 500.0 * time.delta_seconds();
            }
        }
    }
}

pub fn destroy_scene(
    mut commands: Commands,
    scene_element_query: Query<Entity, With<SceneElement>>
) {
    for element_entity in scene_element_query.iter() {
        commands.entity(element_entity).despawn();
    }
    commands.insert_resource(ShipLayout::default());
    commands.insert_resource(PlayerLoot::default());
}

pub fn update_difficulty(
    mut difficulty: ResMut<DifficultyScaling>,
    time: Res<Time>
) {
    difficulty.time_played += time.delta_seconds();
    println!("time played: {}", difficulty.time_played);
    println!("diff mod: {}", difficulty.get_difficulty_modifier());
    println!("enemy spawnrate: {}", difficulty.get_enemy_spawnrate());
    println!("enemy droprate: {}", difficulty.get_loot_droprate());
    println!("---------------------")
}
