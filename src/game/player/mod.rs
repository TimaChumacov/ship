use bevy::prelude::*;

pub mod components;

pub mod systems;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
        .add_system(player_movement);
    }
}