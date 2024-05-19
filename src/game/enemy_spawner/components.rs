use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {}

#[derive(Component)]
pub struct ChaseBehavior {
    pub speed: f32,
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer { timer: Timer::from_seconds(3.0, TimerMode::Repeating) }
    }
}