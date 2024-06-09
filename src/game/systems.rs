use bevy::prelude::*;
use super::components::*;
use crate::game::enemies::components::Enemy;

pub fn update_destructibles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    destructibles: Query<(Entity, &Destructible, &Transform, Option<&Enemy>)>
) {
    for (destruct_entity, destruct_stats, enemy_transform, enemy) in destructibles.iter() {
        if destruct_stats.hp <= 0 {
            commands.entity(destruct_entity).despawn();

            if enemy.is_some() {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_translation(enemy_transform.translation),
                        texture: asset_server.load("sprites/loot.png"),
                        ..default()
                    },
                    Loot {
                        is_targeted: false,
                    }
                ));
            }
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

pub fn collision_logic(
    mut collider_query: Query<(&mut Transform, &Collider)>,
    time: Res<Time>
) {
    let mut combinations = collider_query.iter_combinations_mut();
    while let Some([(mut transform_1, collider_1), (mut transform_2, collider_2)]) = combinations.fetch_next() {
        if transform_1.translation.distance(transform_2.translation) < collider_1.radius + collider_2.radius {
            if collider_1.collision_response == CollisionResponse::Moves && collider_2.collision_response == CollisionResponse::Moves {
                let dir_away = (transform_1.translation - transform_2.translation).normalize();
                transform_1.translation += dir_away * 100.0 * time.delta_seconds();
                transform_2.translation -= dir_away * 100.0 * time.delta_seconds();
            }
        }
    }
    
}