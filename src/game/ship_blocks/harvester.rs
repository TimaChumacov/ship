use bevy::prelude::*;
use super::components::{Block, Spawn};

#[derive(Component)]
pub struct Harvester {}

impl Spawn for Harvester {
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
            Block {},
            Harvester {},
        )).with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    texture: asset_server.load("sprites/grappler.png"),
                    ..default()
                },
                Grappler {}
            ));
        });
    }

    fn spawn_ui(
        parent: &mut ChildBuilder, 
        asset_server: &Res<AssetServer>
    ) {
        parent.spawn(
            ImageBundle {
                image: asset_server.load("sprites/grappler.png").into(),
                ..default()
            }
        );
    }
}

#[derive(Component)]
pub struct Grappler {}