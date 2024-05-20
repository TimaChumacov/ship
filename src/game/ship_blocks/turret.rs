use bevy::prelude::*;
use super:: components::Block;

#[derive(Component)]
pub struct Turret {}

impl Turret {
    pub fn spawn(
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
    ) {
        parent.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(0.0, 32.0, 0.0),
                texture: asset_server.load("sprites/turret.png"),
                ..default()
            },
            Block {},
            Turret {},
        ));
    }
}