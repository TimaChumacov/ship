use bevy::prelude::*;
use crate::general::states::PauseState;

pub mod systems;
use systems::*;

pub mod components;
pub mod styles;

pub mod layout;
use layout::*;
pub struct ShipEditUiPlugin;

impl Plugin for ShipEditUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            show_or_hide_ui, 
            interact_with_ui_blocks.run_if(in_state(PauseState::Paused)),
            interact_with_ui_loot.run_if(in_state(PauseState::Paused)),
            deselect_button.run_if(in_state(PauseState::Paused)),
            rotate_loot.run_if(in_state(PauseState::Paused)),
        ));
    }
}