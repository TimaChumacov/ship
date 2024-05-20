use bevy::prelude::*;

pub mod components;

pub mod player;
use player::PlayerPlugin;

pub mod ship_blocks;
use ship_blocks::BaseBlockPlugin;

pub mod enemies;
use enemies::EnemySpawnerPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(EnemySpawnerPlugin);
    }
}