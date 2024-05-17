use bevy::prelude::*;

pub mod base_block;
use base_block::BaseBlockPlugin;

pub mod enemy_spawner;
use enemy_spawner::EnemySpawnerPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BaseBlockPlugin);
    }
}