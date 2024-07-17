use bevy::prelude::*;
use crate::general::states::{AppState, PauseState};

pub mod systems;
use systems::*;

pub mod resources;
use resources::*;

pub mod components;
pub mod styles;

pub mod layout;
use layout::*;
pub struct ShipEditUiPlugin;

impl Plugin for ShipEditUiPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Selection>()
        .add_systems(Update, show_or_hide_ui.run_if(in_state(AppState::Game)))
        .add_systems(Update, (
            interact_with_ui_blocks,
            check_dragg_and_dropped,
            interact_with_ui_loot,
            deselect_button,
            rotate_loot,
            update_draggable
        ).run_if(in_state(PauseState::Paused)));
    }
}