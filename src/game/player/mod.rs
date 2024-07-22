use bevy::prelude::*;
use crate::general::states::{AppState, PauseState};

pub mod systems;
use systems::*;

use self::components::ShipLayout;

pub mod components;

pub mod player_loot;
use player_loot::PlayerLoot;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShipLayout>()
            .init_resource::<PlayerLoot>()
            .add_systems( OnEnter(AppState::Game), spawn_player)
            .add_systems(Update, player_movement.run_if(in_state(PauseState::Running)));
    }
}