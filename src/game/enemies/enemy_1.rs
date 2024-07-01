use bevy::prelude::*;
use crate::{game::{components::Destructible, Collider}, general::components::SceneElement};
use super::components::{ChaseBehavior, Enemy, Melee};

#[derive(Component)]
pub struct Enemy1 {}

impl Enemy1 {
    pub fn spawn(
        spawn_pos: Vec3,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(spawn_pos),
                texture: asset_server.load("sprites/enemy.png"),
                ..default()
            },
            SceneElement {},
            Enemy {},
            Destructible {
                hp: 3,
                max_hp: 3,
                time_spent_red: 0.0,
            },
            ChaseBehavior {
                speed: 40.0,
            },
            Melee {
                damage: 1,
                attack_cooldown: 1.0,
                cooldown_left: 0.0,
            },
            Collider::default()
        ));
    }
}