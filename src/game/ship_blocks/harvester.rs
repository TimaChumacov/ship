use bevy::prelude::*;
use crate::game::{Collider, Destructible};

use super::{components::Block, traits::*};

#[derive(Component, Clone, PartialEq)]
pub struct Harvester {
    pub rotation: f32,
    pub deployed_grappler: bool,
}

#[derive(Component)]
pub struct Grappler {
    pub target: Option<Entity>,
    pub base: Option<Entity>,
    pub is_returning: bool,
    pub grabbed_loot: bool,
}

#[derive(Component)]
pub struct Wire {}

impl Default for Harvester {
    fn default() -> Self {
        Harvester {
            rotation: 0.0,
            deployed_grappler: false,
        }
    }
}

impl Harvester {
    pub fn spawn_child_grappler(
        &self,
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
    ) {
        parent.spawn(
            SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 1.0)
                                     .with_rotation(Quat::from_rotation_z(-self.rotation.to_radians())),
                texture: asset_server.load("sprites/grappler.png"),
                ..default()
            }
        );
    }

    pub fn spawn_global_grappler(
        &self,
        spawn_pos: Vec3,
        target: Option<Entity>,
        base: Option<Entity>,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    ) {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(spawn_pos.xy().extend(1.0))
                                     .with_rotation(Quat::from_rotation_z(-self.rotation.to_radians())),
                texture: asset_server.load("sprites/grappler.png"),
                ..default()
            },
            Grappler {
                target: target,
                base: base,
                is_returning: false,
                grabbed_loot: false,
            }
        )).with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(1.0, 0.0, 1.0)),
                    texture: asset_server.load("sprites/grappler_wire.png"),
                    ..default()
                },
                Wire {}
            ));
        });
    }
}

impl Spawn for Harvester {
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
            Block {
                x: x,
                y: y,
            },
            Harvester::default(),
            Destructible {
                hp: 3,
                time_spent_red: 0.0,
            },
            Collider {
                collision_response: crate::game::CollisionResponse::Stays,
                ..default()
            }
        )).with_children(|parent| {
            self.spawn_child_grappler(parent, asset_server);
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