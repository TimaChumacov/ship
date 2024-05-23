use bevy::prelude::*;

pub mod components;
pub mod styles;

pub mod layout;
use layout::*;
pub struct ShipEditUiPlugin;

impl Plugin for ShipEditUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(show_or_hide_ui);
    }
}