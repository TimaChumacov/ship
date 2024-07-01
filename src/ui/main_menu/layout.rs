use bevy::prelude::*;
use super::components::*;

use super::styles::*;

pub fn spawn_ui(
    mut commands: Commands
) {
    println!("spawn ui function");
    // screen wrapp
    commands.spawn((
        NodeBundle {
            style: wrapp(),
            background_color: BG_COLOR.into(),
            ..default()
        },
        MainMenuScreen {}
    // block with the entire menu inside
    )).with_children(|parent| {
        parent.spawn((
            NodeBundle {
                style: menu(),
                background_color: MENU_COLOR.into(),
                ..default()
            },
        // play button
        )).with_children(|parent| {
            parent.spawn((
                ButtonBundle {
                    style: button(),
                    background_color: BLOCK_COLOR.into(),
                    ..default()
                },
                PlayButton {}
            // text inside the button
            )).with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection::new("PLAY", button_text())],
                        ..default()
                    },
                    ..default()
                });
            });
        });
    });
}