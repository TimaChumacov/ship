use bevy::prelude::*;
use super::components::Block;

#[derive(Component)]
pub struct Harvester {}

impl Harvester {
    pub fn spawn(
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
    ) {
        parent.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(32.0, 0.0, 0.0),
                texture: asset_server.load("sprites/base.png"),
                ..default()
            },
            Block {},
            Harvester {},
        )).with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    texture: asset_server.load("sprites/grappler.png"),
                    ..default()
                },
                Grappler {}
            ));
        });
    }
}

#[derive(Component)]
pub struct Grappler {}