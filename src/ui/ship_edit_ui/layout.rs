use bevy::log::tracing_subscriber::fmt::TestWriter;
use bevy::{a11y::accesskit::Node, prelude::*, transform::commands};
use crate::general::states::PauseState;
use super::components::*;
use super::styles::*;
use crate::game::player::components::*;

pub fn show_or_hide_ui(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<PauseState>>,
    mut next_state: ResMut<NextState<PauseState>>,
    ui_query: Query<Entity, With<ShipEditMenu>>,
    mut ship_layout: ResMut<ShipLayout>,
    asset_server: Res<AssetServer>,
    player_query: Query<Entity, With<Player>>,
    player_res: Res<PlayerResource>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            PauseState::Running => {
                next_state.set(PauseState::Paused);
                spawn_ui(commands, ship_layout.into(), asset_server, player_res)
            }
            PauseState::Paused => {
                next_state.set(PauseState::Running);
                let ui_entity = ui_query.single();
                commands.entity(ui_entity).despawn_recursive();
                if !ship_layout.old_blocks_empty() {
                    ship_layout.update_ship(commands, &player_query, &asset_server)
                }
            }    
        }
    }
}

fn spawn_ui(
    mut commands: Commands,
    ship_layout: Res<ShipLayout>,
    asset_server: Res<AssetServer>,
    player_res: Res<PlayerResource>,
) {
    commands.spawn((
        NodeBundle {
            style: wrapp(),
            background_color: BLOCK_COLOR.into(),
            ..default()
        },
        ShipEditMenu {}
    )).with_children(|parent| {
        // --- left info menu ---
        parent.spawn(
            NodeBundle {
                style: info_menu(),
                background_color: MAIN_COLOR.into(),
                ..default()
            }
        ).with_children(|parent| {
            // --- Title: Block Name ---
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new("Title", title())],
                        ..default()
                    },
                    ..default()
                }
            );
            // --- Image: Block Sprite ---
            parent.spawn((
                NodeBundle {
                    style: block(),
                    background_color: BLOCK_COLOR.into(),
                    ..default()
                },
                SelectedLootUi {}
            ));
            // --- Button: deselect selected block ---
            parent.spawn((
                ButtonBundle {
                    style: unselect_button(),
                    background_color: BLOCK_COLOR.into(),
                    ..default()
                },
                DeselectButton {}
            )).with_children(|parent| {
                parent.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec![TextSection::new("Deselect", text())],
                            ..default()
                        },
                        ..default()
                    }
                );
            });
            // --- Text: Block description ---
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new("", text())],
                        ..default()
                    },
                    ..default()
                }
            );
        });
        // --- grid edit menu ---
        parent.spawn(
            NodeBundle {
                style: grid_menu(),
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
                            background_color: BLOCK_COLOR.into(),
                            ..default()
                        },
                        UiBlock {
                            x: a_usize,
                            y: b_usize,
                        },
                    )).with_children(|parent| {
                        if let Some(y) = y {
                            y.spawn_ui(parent, &asset_server);
                        } 
                    });
                };
            };
        });
        // --- right selection menu ---
        parent.spawn((
            NodeBundle {
                style: loot_menu(),
                background_color: MAIN_COLOR.into(),
                ..default()
            },
            LootMenu {}
        )).with_children(|parent| {
            player_res.spawn_loot_ui(parent, &asset_server);
        });
    });
}