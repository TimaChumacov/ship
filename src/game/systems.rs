use bevy::prelude::*;
use super::components::{Destructible, Loot};
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