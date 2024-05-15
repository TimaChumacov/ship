use bevy::prelude::*;

mod general;
use general::GeneralPlugin;

mod game;
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set( 
            ImagePlugin::default_nearest(), 
        )) 
        .add_plugin(GeneralPlugin)
        .add_plugin(GamePlugin)
        .run()
}