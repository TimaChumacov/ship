use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use crate::game::ship_blocks::traits::get_generic_info;
use crate::game::ship_blocks::traits::get_generic_stats;
use crate::game::ship_blocks::traits::get_generic_title;
use crate::general::states::PauseState;
use super::components::*;
use super::styles::*;
use crate::game::player::components::*;
use crate::game::ship_blocks::traits::Spawn;

pub fn show_or_hide_ui(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<PauseState>>,
    mut next_state: ResMut<NextState<PauseState>>,
    ui_query: Query<Entity, With<ShipEditMenu>>,
    ship_layout: ResMut<ShipLayout>,
    asset_server: Res<AssetServer>,
    player_query: Query<Entity, With<Player>>,
    mut player_loot: ResMut<PlayerLoot>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            PauseState::Running => {
                next_state.set(PauseState::Paused);
                spawn_ui(commands, ship_layout.into(), asset_server, player_loot.into())
            }
            PauseState::Paused => {
                if ship_layout.is_core_placed() {
                    next_state.set(PauseState::Running);
                    player_loot.selected_loot_index = None;
                    let ui_entity = ui_query.single();
                    commands.entity(ui_entity).despawn_recursive();
                    if !ship_layout.old_blocks_empty() {
                        ship_layout.reset_ship(&mut commands, &player_query, &asset_server)
                    }
                }
            }    
        }
    }
}

fn spawn_ui(
    mut commands: Commands,
    ship_layout: Res<ShipLayout>,
    asset_server: Res<AssetServer>,
    player_res: Res<PlayerLoot>,
) {
    commands.spawn((
        NodeBundle {
            style: wrapp(),
            ..default()
        },
        ShipEditMenu {}
    )).with_children(|parent| {
        // --- left info menu ---
        parent.spawn(
            NodeBundle {
                style: info_menu(),
                background_color: Color::WHITE.into(),
                ..default()
            }
        ).with_children(|parent| {
            // --- Bg image ---
            parent.spawn(
                ImageBundle {
                    image: asset_server.load("sprites/ui_menu.png").into(),
                    style: absolute(),
                    ..default()
                }
            );
            // --- Title: Block Name ---
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(get_generic_title(), title())],
                        ..default()
                    },
                    ..default()
                },
                SelectedLootTitle {}
            ));
            // --- Image: Block Sprite ---
            parent.spawn((
                NodeBundle {
                    style: block(),
                    background_color: BLOCK_COLOR.into(),
                    ..default()
                },
                SelectedLootIcon {}
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
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(get_generic_stats(), stats_text()),
                            TextSection::new(get_generic_info(), text())
                        ],
                        ..default()
                    },
                    ..default()
                },
                SelectedLootDescription {}
            ));
        });
        // --- grid edit menu ---
        parent.spawn((
            NodeBundle {
                style: grid_menu(),
                background_color: MAIN_COLOR.into(),
                ..default()
            },
            Gridmenu {}
        )).with_children(|parent| {
            for (a_usize, x) in ship_layout.blocks.iter().enumerate() {
                for (b_usize, y) in x.iter().enumerate() {
                    //let (a , b) = (a_usize as f32, b_usize as f32);
                    parent.spawn((
                        ButtonBundle {
                            style: block(),
                            border_color: BLOCK_COLOR.into(),
                            //background_color: BLOCK_COLOR.into(),
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
                        parent.spawn((
                            ImageBundle {
                                image: asset_server.load("sprites/select_frame.png").into(),
                                style: selection_frame(),
                                z_index: ZIndex::Global(2),
                                visibility: Visibility::Hidden,
                                ..default()
                            },
                            BlockHoverFrame {}
                        ));
                        parent.spawn(
                            ImageBundle {
                                image: asset_server.load("sprites/ui_block.png").into(),
                                style: absolute(),
                                z_index: ZIndex::Global(1),
                                ..default()
                            },
                        );
                    });
                };
            };
        });
        // --- Loot menu ---
        parent.spawn(
            NodeBundle {
                style: loot_menu(),
                ..default()
            }
        ).with_children(|parent| {
            // --- Bg image ---
            parent.spawn(
                ImageBundle {
                    image: asset_server.load("sprites/ui_menu.png").into(),
                    style: absolute(),
                    ..default()
                }
            );
            // --- Wrapp for loot grid ---
            parent.spawn((
                NodeBundle {
                    style: loot_grid_wrapp(),
                    ..default()
                },
                LootMenu {}
            ));
            player_res.spawn_loot_ui(parent, &asset_server);
        });
    });
}