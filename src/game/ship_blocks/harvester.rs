use bevy::prelude::*;
use super::{components::Block, traits::*};

#[derive(Component, Clone, PartialEq)]
pub struct Harvester {
    pub rotation: f32,
}

#[derive(Component)]
pub struct Grappler {
    pub is_looting: bool,
    pub grabbed_loot: bool,
}

impl Default for Harvester {
    fn default() -> Self {
        Harvester {
            rotation: 0.0
        }
    }
}

impl Spawn for Harvester {
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
            Harvester::default(),
        )).with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 1.0)
                                         .with_rotation(Quat::from_rotation_z(-self.rotation.to_radians())),
                    texture: asset_server.load("sprites/grappler.png"),
                    ..default()
                },
                Grappler {
                    is_looting: false,
                    grabbed_loot: false,
                }
            ));
        });
    }

    fn spawn_ui(
        &self,
        parent: &mut ChildBuilder, 
        asset_server: &Res<AssetServer>
    ) {
        parent.spawn(
            ImageBundle {
                transform: Transform::from_rotation(Quat::from_rotation_z(self.rotation.to_radians())),
                image: asset_server.load("sprites/grappler.png").into(),
                ..default()
            }
        );
    }
}

impl Rotate for Harvester {
    fn get_rotation(&self) -> f32 {
        self.rotation
    }

    fn rotate_90_right(&mut self) {
        self.rotation += 90.0
    }
}