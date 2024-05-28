use bevy::prelude::*;
use crate::general::states::PauseState;

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
        app.add_plugins((
            PlayerPlugin, 
            EnemySpawnerPlugin, 
            ShipBlocksPlugin
        )
    )
        .add_systems(Update, (update_destructibles).run_if(in_state(PauseState::Running)));
    }
}