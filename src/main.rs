use bevy::prelude::*;

mod general;
use general::GeneralPlugin;

mod game;
use game::GamePlugin;

mod ui;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set( 
            ImagePlugin::default_nearest(), 
        )) 
        .add_plugin(GeneralPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(UiPlugin)
        .run()
}