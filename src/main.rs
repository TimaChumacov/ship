use bevy::{log::LogPlugin, prelude::*};

mod general;
use bevy_kira_audio::AudioPlugin;
use general::GeneralPlugin;

mod game;
use game::GamePlugin;

mod ui;
use ui::UiPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.078, 0.114, 0.153)))
        .add_plugins((
            DefaultPlugins.set( 
            ImagePlugin::default_nearest(), 
            ).set(LogPlugin {
                filter: "off".into(),
                level: bevy::log::Level::DEBUG,
                ..default()
            }), 
            AudioPlugin
    )) 
        .add_plugins(GeneralPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(UiPlugin)
        .run();
}