use bevy::prelude::*;
use crate::general::states::PauseState;

mod systems;
use systems::*;

pub mod components;
use components::EnemySpawnTimer;

pub mod enemy_1;

pub struct EnemySpawnerPlugin;

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(Update, (
                spawn_enemies, 
                enemy_movement,
                enemy_collides_block,
                enemy_death,
            ).run_if(in_state(PauseState::Running)));
    }
}