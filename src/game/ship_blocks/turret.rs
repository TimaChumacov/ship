use bevy::prelude::*;
use super:: components::{Block, Spawn};

#[derive(Component)]
pub struct Turret {}

impl Spawn for Turret {
    fn spawn(
        spawn_pos: Vec3,
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
    ) {
        parent.spawn((
            SpriteBundle {
                transform: Transform::from_translation(spawn_pos),
                texture: asset_server.load("sprites/turret.png"),
                ..default()
            },
            Block {},
            Turret {},
        ));
    }

    fn spawn_ui(
        parent: &mut ChildBuilder, 
        asset_server: &Res<AssetServer>
    ) {
        parent.spawn(
            ImageBundle {
                image: asset_server.load("sprites/turret.png").into(),
                ..default()
            }
        );
    }
}

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