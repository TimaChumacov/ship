use bevy::prelude::*;

pub mod components;

pub mod systems;
use systems::*;

pub struct GeneralPlugin;

impl Plugin for GeneralPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
           .add_startup_system(spawn_crosshair)
           .add_system(crosshair_follow_mouse);
    }
}