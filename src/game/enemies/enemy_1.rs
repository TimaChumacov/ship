use bevy::prelude::*;
use crate::game::components::Destructible;
use super::components::{Enemy, ChaseBehavior};

#[derive(Component)]
pub struct Enemy1 {}

impl Enemy1 {
    pub fn spawn(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(600.0, 500.0, 0.0),
                texture: asset_server.load("sprites/enemy.png"),
                ..default()
            },
            Enemy {},
            Destructible {
                hp: 1,
            },
            ChaseBehavior {
                speed: 40.0,
            }
        ));
    }
}