use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {}

#[derive(Component)]
pub struct ChaseBehavior {
    pub speed: f32,
}

#[derive(Component)]
pub struct Melee {
    pub damage: i8,
    pub attack_cooldown: f32,
    pub cooldown_left: f32,
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer { timer: Timer::from_seconds(0.8, TimerMode::Repeating) }
    }
}