use bevy::prelude::*;
use super::components::{Block, Spawn};

#[derive(Component)]
pub struct Core {}

impl Spawn for Core {
    fn spawn(
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
            Block {}
        )).with_children(|parent| {
            parent.spawn(
                SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    texture: asset_server.load("sprites/core.png"),
                    ..default()
                }
            );
        });
    }
}