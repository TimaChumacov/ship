use bevy::prelude::*;

pub mod components;

pub mod states;
use states::*;

pub mod systems;
use systems::*;

pub struct GeneralPlugin;

impl Plugin for GeneralPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PauseState>()
            .add_systems(PreStartup, (spawn_camera, spawn_background))
            .add_systems(Update, (
                background_follow,
                follow_player
            ).run_if(in_state(PauseState::Running)));
    }
}