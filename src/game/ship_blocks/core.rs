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
                hp: 120,
                max_hp: 120,
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
                z_index: ZIndex::Local(2),
                ..default()
            }
        );
    }
}

impl Description for Core {
    fn get_info(&self) -> String {
        format!("This is the core of the ship. It has 10 max hp and when it's destroyed, you lose. The ship always has to have a core somewhere, you can't remove it but you can move it around.")
    }

    fn get_title(&self) -> String {
        format!("Core")
    }
}
