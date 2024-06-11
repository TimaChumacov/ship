use bevy::prelude::*;
use super::{components::*, ship_blocks::components::Block};
use crate::game::enemies::components::Enemy;

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
                let dir_away = (transform_1.translation - transform_2.translation).normalize();
                transform_1.translation += dir_away * 100.0 * time.delta_seconds();
                transform_2.translation -= dir_away * 100.0 * time.delta_seconds();
            } else 
            if collider_1.collision_response == CollisionResponse::Stays && collider_2.collision_response == CollisionResponse::Moves {
                let dir_away = (glob_transform_1.translation() - transform_2.translation);
                let how_close = (collider_1.radius + collider_2.radius) - dir_away.length();
                transform_2.translation -= dir_away.normalize() * how_close * time.delta_seconds();
            } else
            if collider_2.collision_response == CollisionResponse::Stays && collider_1.collision_response == CollisionResponse::Moves {
                let dir_away = (transform_1.translation - glob_transform_2.translation());
                let how_close = (collider_1.radius + collider_2.radius) - dir_away.length();
                transform_1.translation += dir_away.normalize() * how_close * time.delta_seconds();
            }
        }
    }
}