use bevy::prelude::*;
use crate::general::states::{AppState, PauseState};

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
        .add_event::<CollisionEvent>()
        .add_event::<EnemyDeathEvent>()
        .add_event::<BlockDestructionEvent>()
        .add_systems(Update, (
            update_destructibles,
            animate_loot,
            trigger_animation,
            damaged_animation,
            collision_detection,
            collision_physics_logic
        ).run_if(in_state(PauseState::Running)))
        .add_systems(OnExit(AppState::Game), destroy_scene);
    }
}