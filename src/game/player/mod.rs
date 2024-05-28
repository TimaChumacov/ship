use bevy::prelude::*;
use crate::general::states::PauseState;

pub mod systems;
use systems::*;

use self::components::ShipLayout;

pub mod components;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShipLayout>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement.run_if(in_state(PauseState::Running)));
    }
}