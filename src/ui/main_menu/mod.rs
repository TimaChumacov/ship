use bevy::prelude::*;

pub mod layout;
use layout::*;
use systems::play_button;

pub mod styles;
pub mod components;

use crate::general::states::AppState;

pub mod systems;
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_ui)
           .add_systems(Update, play_button);
    }
}