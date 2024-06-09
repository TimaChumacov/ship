use bevy::prelude::*;
use crate::general::states::PauseState;

pub mod components;
use components::*;

mod systems;
use systems::*;

pub mod player;
use player::PlayerPlugin;

pub mod ship_blocks;
use ship_blocks::ShipBlocksPlugin;

pub mod enemies;
use enemies::EnemySpawnerPlugin;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerPlugin, 
            EnemySpawnerPlugin, 
            ShipBlocksPlugin
        ))
        .add_event::<DamagedEvent>()
        .add_systems(Update, (
            update_destructibles,
            trigger_animation,
            damaged_animation,
            collision_logic
        ).run_if(in_state(PauseState::Running)));
    }
}