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
        )).with_children(|parent| {
            // Guide
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new("This a prototype of a top-down swarm survival game. You play as ship that consists of different parts that have their own functionality. Move with [W A S D] and rotate your ship with [Q] and [E], it's VERY important to rotate. By pressing [Esc] you enter the ship editing menu, where you can customize your ship. Enemies sometimes drop ship parts on death. Collect the droped loot by coming close to it. If the the Core of the ship is destroyed, you die. (Background and SFX are not mine)", button_text())],
                    ..default()
                },
                ..default()
            });
            // play button
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