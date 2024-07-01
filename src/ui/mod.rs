use bevy::prelude::*;

pub mod ship_edit_ui;
use ship_edit_ui::ShipEditUiPlugin;

pub mod main_menu;
use main_menu::MainMenuPlugin;
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ShipEditUiPlugin,
            MainMenuPlugin
        ));
    }
}