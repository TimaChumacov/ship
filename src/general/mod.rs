use bevy::prelude::*;

pub mod components;

pub mod systems;
use systems::*;

pub struct GeneralPlugin;

impl Plugin for GeneralPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}