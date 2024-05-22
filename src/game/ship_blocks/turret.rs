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