use bevy::prelude::*;

pub mod components;

pub mod systems;
use systems::*;

pub struct BaseBlockPlugin;

impl Plugin for BaseBlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(rotate_grappler);
    }
}