use bevy::prelude::*;

pub mod ship_edit_ui;
use ship_edit_ui::ShipEditUiPlugin;
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ShipEditUiPlugin);
    }
}