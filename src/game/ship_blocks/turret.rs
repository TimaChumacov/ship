use bevy::prelude::*;
use super::{components::Block, traits::*};

#[derive(Component, Clone, PartialEq)]
pub struct Turret {
    pub rotation: f32,
}

impl Default for Turret {
    fn default() -> Self {
        Turret {
            rotation: 90.0,
        }
    }
}

impl Spawn for Turret {
    fn spawn(
        &self,
        spawn_pos: Vec3,
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
    ) {
        parent.spawn((
            SpriteBundle {
                transform: Transform::from_translation(spawn_pos)
                                     .with_rotation(Quat::from_rotation_z(self.rotation.to_radians())),
                texture: asset_server.load("sprites/turret.png"),
                ..default()
            },
            Block {},
            Turret::default(),
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