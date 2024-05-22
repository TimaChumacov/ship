use bevy::prelude::*;

pub mod components;
mod systems;

pub mod player;
use player::PlayerPlugin;

pub mod ship_blocks;
use ship_blocks::ShipBlocksPlugin;

pub mod enemies;
use enemies::EnemySpawnerPlugin;

use self::systems::update_destructibles;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(EnemySpawnerPlugin)
            .add_plugin(ShipBlocksPlugin)
            .add_system(update_destructibles);
    }
}