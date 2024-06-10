use bevy::prelude::*;
use crate::game::{Collider, Destructible};

use super::{components::Block, traits::*};

#[derive(Component, Clone, PartialEq)]
pub struct Turret {
    pub rotation: f32,
}

impl Default for Turret {
    fn default() -> Self {
        Turret {
            rotation: 0.0,
        }
    }
}

impl Spawn for Turret {
    fn spawn(
        &self,
        x: usize,
        y: usize,
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
    ) {
        parent.spawn((
            SpriteBundle {
                transform: Transform::from_translation(Block::location_by_index(x, y))
                                     .with_rotation(Quat::from_rotation_z(-self.rotation.to_radians())),
                texture: asset_server.load("sprites/turret.png"),
                ..default()
            },
            Block {
                x: x,
                y: y,
            },
            Turret {
                rotation: self.rotation,
            },
            Destructible {
                hp: 3,
                time_spent_red: 0.0,
            },
            Collider {
                collision_response: crate::game::CollisionResponse::Stays,
                ..default()
            }
        ));
    }

    fn spawn_ui(
        &self,
        parent: &mut ChildBuilder, 
        asset_server: &Res<AssetServer>
    ) {
        parent.spawn(
            ImageBundle {
                transform: Transform::from_rotation(Quat::from_rotation_z(self.rotation.to_radians())),
                image: asset_server.load("sprites/turret.png").into(),
                ..default()
            }
        );
    }
}

impl Rotate for Turret {
    fn get_rotation(&self) -> f32 {
        self.rotation
    }

    fn rotate_90_right(&mut self) {
        self.rotation += 90.0
    }
}

// --- Projectiles gonna have own space later ---
#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
    pub damage: i8,
    pub lifetime: f32,
    pub is_offseted: bool,
}

impl Default for Bullet {
    fn default() -> Self {
        Bullet {
            speed: 150.0,
            damage: 1,
            lifetime: 1.0,
            is_offseted: false,
        }
    }
}

#[derive(Resource)]
pub struct TurretTimer {
    pub timer: Timer,
}

impl Default for TurretTimer {
    fn default() -> Self {
        TurretTimer { timer: Timer::from_seconds(0.3, TimerMode::Repeating) }
    }
}