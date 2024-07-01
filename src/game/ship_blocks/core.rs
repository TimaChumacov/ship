use bevy::prelude::*;
use crate::{game::{Collider, Destructible}, general::components::SceneElement};

use super::{components::Block, traits::{Description, Spawn}};

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
        x: usize,
        y: usize,
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
    ) {
        parent.spawn((
            SpriteBundle {
                transform: Transform::from_translation(Block::location_by_index(x, y)),
                texture: asset_server.load("sprites/base.png"),
                ..default()
            },
            SceneElement {},
            Block {
                x: x,
                y: y,
            },
            Core::default(),
            Destructible {
                hp: 10,
                max_hp: 10,
                time_spent_red: 0.0,
            },
            Collider {
                collision_response: crate::game::CollisionResponse::Stays,
                ..default()
            }
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

impl Description for Core {}
