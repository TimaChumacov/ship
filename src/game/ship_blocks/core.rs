use bevy::prelude::*;
use crate::game::{Collider, Destructible};

use super::{components::Block, traits::Spawn};

#[derive(Component, Clone, PartialEq)]
pub struct Core {}

impl Default for Core {
    fn default() -> Self {
        Core {}
    }
}

impl Spawn for Core {
    fn spawn(
        &self,
        spawn_pos: Vec3,
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
    ) {
        parent.spawn((
            SpriteBundle {
                transform: Transform::from_translation(spawn_pos),
                texture: asset_server.load("sprites/base.png"),
                ..default()
            },
            Block {},
            Core::default(),
            Destructible {
                hp: 10,
                time_spent_red: 0.0,
            },
            Collider::default()
        )).with_children(|parent| {
            parent.spawn(
                SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    texture: asset_server.load("sprites/core.png"),
                    ..default()
                }
            );
        });
    }

    fn spawn_ui(
        &self,
        parent: &mut ChildBuilder, 
        asset_server: &Res<AssetServer>
    ) {
        parent.spawn(
            ImageBundle {
                image: asset_server.load("sprites/core.png").into(),
                ..default()
            }
        );
    }
}
