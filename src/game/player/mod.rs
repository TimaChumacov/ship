use bevy::prelude::*;

pub mod systems;
use systems::*;

use self::components::ShipLayout;

pub mod components;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShipLayout>()
            .add_startup_system(spawn_player)
            .add_system(player_movement);
    }
}