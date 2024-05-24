use bevy::prelude::*;

mod systems;
use systems::*;

pub mod components;
use components::EnemySpawnTimer;

pub mod enemy_1;

pub struct EnemySpawnerPlugin;

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(Update, (spawn_enemies, enemy_movement));
    }
}