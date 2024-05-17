use bevy::prelude::*;

mod systems;
mod timer;

mod enemy_1;

pub struct EnemySpawnerPlugin;

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_system(spawn_enemies);
    }
}