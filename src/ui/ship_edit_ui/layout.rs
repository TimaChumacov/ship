use bevy::{a11y::accesskit::Node, prelude::*, transform::commands};
use super::components::*;
use super::styles::*;
use crate::game::player::components::*;

pub fn show_or_hide_ui(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    ui_query: Query<Entity, With<ShipEditMenu>>,
    mut ship_layout: ResMut<ShipLayout>,
    asset_server: Res<AssetServer>,
    player_query: Query<Entity, With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if let Ok(ui_entity) = ui_query.get_single() {
            commands.entity(ui_entity).despawn_recursive();
            if !ship_layout.old_blocks_empty() {
                ship_layout.update_ship(commands, &player_query, &asset_server)
            }
        } else {
            spawn_ui(commands, ship_layout.into(), asset_server)
        }
    }
}

fn spawn_ui(
    mut commands: Commands,
    ship_layout: Res<ShipLayout>,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        NodeBundle {
            style: wrapp(),
            background_color: WRAPP_BG_COLOR.into(),
            ..default()
        },
        ShipEditMenu {}
    )).with_children(|parent| {
        parent.spawn(
            NodeBundle {
                style: menu(),
                background_color: MAIN_COLOR.into(),
                ..default()
            }
        ).with_children(|parent| {
            for (a_usize, x) in ship_layout.blocks.iter().enumerate() {
                for (b_usize, y) in x.iter().enumerate() {
                    let (a , b) = (a_usize as f32, b_usize as f32);
                    parent.spawn((
                        ButtonBundle {
                            style: block(),
                            background_color: WRAPP_BG_COLOR.into(),
                            ..default()
                        },
                        UiBlock {
                            x: a_usize,
                            y: b_usize,
                        },
                    )).with_children(|parent| {
                        // if ship_layout.blocks[b_usize][a_usize].is_some() {
                        //     ship_layout.blocks[b_usize][a_usize].unwrap().spawn_ui(parent, &asset_server);
                        // }
                        if let Some(y) = y {
                            //if ship_layout.blocks[b_usize][a_usize].is_some() {};
                            y.spawn_ui(parent, &asset_server);
                        } 
                    });
                };
            };
        });
    });
}