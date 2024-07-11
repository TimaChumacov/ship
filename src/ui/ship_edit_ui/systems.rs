use bevy::prelude::*;
use crate::game::player::components::{PlayerLoot, ShipLayout};
use crate::game::ship_blocks::traits::{Rotate, Spawn};
use super::styles::selection_frame;
use super::{components::*, Selection};

pub fn interact_with_ui_blocks(
    mut commands: Commands,
    mut selection: ResMut<Selection>,
    asset_server: Res<AssetServer>,
    mut ship_layout: ResMut<ShipLayout>,
    mut player_loot: ResMut<PlayerLoot>,
    selected_loot_icon: Query<Entity, With<SelectedLootIcon>>,
    selected_loot_text: Query<&mut Text, With<SelectedLootDescription>>,
    selected_loot_title: Query<&mut Text, (With<SelectedLootTitle>, Without<SelectedLootDescription>)>,
    loot_menu_query: Query<Entity, With<LootMenu>>,
    mut button_query: Query<
        (&Interaction, &mut BorderColor, &UiBlock, Entity),
        Changed<Interaction>
    >,
) {
    if let Ok((interaction, mut border_color, ui_block, block_entity)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *border_color = Color::PURPLE.into();
                if ship_layout.old_blocks_empty() {
                    ship_layout.old_blocks = ship_layout.blocks.clone();
                }
                commands.entity(block_entity).despawn_descendants();
                let mut pressed_block = ship_layout.blocks[ui_block.x][ui_block.y].clone();
                if pressed_block == None {
                    if let Some(selected_block) = player_loot.get_selected_loot() {
                        pressed_block = Some(selected_block.clone());
                        commands.entity(block_entity).with_children(|parent| {
                            selected_block.spawn_ui(parent, &asset_server)
                        });
                        player_loot.remove_used_loot(&mut commands, selected_loot_icon, selected_loot_text, selected_loot_title);
                        player_loot.redraw_loot_ui(&mut commands, &loot_menu_query, &asset_server);
                    }
                } else {
                    player_loot.put_block_in_loot(&pressed_block.unwrap());
                    pressed_block = None;
                }
                ship_layout.blocks[ui_block.x][ui_block.y] = pressed_block;
                player_loot.redraw_loot_ui(&mut commands, &loot_menu_query, &asset_server)
            },
            Interaction::Hovered => {
                selection.hovered_block = Some(block_entity);
                *border_color = Color::RED.into();
            },
            Interaction::None => {
                *border_color = Color::NONE.into();
            }
        }
    }
}

pub fn animate_selection(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    selection: Res<Selection>,
    grid_query: Query<Entity, With<Gridmenu>>,
    mut block_hover_frame_query: Query<&mut Style, With<BlockHoverFrame>>,
) {
    if let Some(hovered_entity) = selection.hovered_block {
        if selection.block_hover_frame.is_none() {
            commands.entity(grid_query.single()).with_children(|parent| {
                parent.spawn((
                    ImageBundle {
                        image: asset_server.load("sprites/hover_frame.png").into(),
                        style: selection_frame(),
                        ..default()
                    },
                    BlockHoverFrame {}
                ));
            });
        }
        //let block_hover_frame = block_hover_frame_query.single_mut();
    }
}

pub fn interact_with_ui_loot(
    mut player_loot: ResMut<PlayerLoot>,
    mut button_query: Query<
    (&Interaction, &mut BorderColor, &LootUiBlock),
    Changed<Interaction>
    >,
    mut commands: Commands,
    selected_loot_ui: Query<Entity, With<SelectedLootIcon>>,
    selected_loot_text: Query<&mut Text, With<SelectedLootDescription>>,
    selected_loot_title: Query<&mut Text, (With<SelectedLootTitle>, Without<SelectedLootDescription>)>,
    asset_server: Res<AssetServer>
) {
    if let Ok((interaction, mut border_color, small_ui_block)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                player_loot.select_loot(small_ui_block.index, &mut commands, selected_loot_ui, selected_loot_text, selected_loot_title, &asset_server);
                //player_loot.redraw_selected_loot(&mut commands, selected_loot_ui, &asset_server);
                *border_color = Color::PURPLE.into();
            },
            Interaction::Hovered => {
                *border_color = Color::RED.into();
            },
            Interaction::None => {
                *border_color = Color::NONE.into();
            }
        }
    }
}

pub fn deselect_button(
    mut commands: Commands,
    mut player_loot: ResMut<PlayerLoot>,
    selected_loot_ui: Query<Entity, With<SelectedLootIcon>>,
    selected_loot_text: Query<&mut Text, With<SelectedLootDescription>>,
    selected_loot_title: Query<&mut Text, (With<SelectedLootTitle>, Without<SelectedLootDescription>)>,
    mut button_query: Query<
    (&Interaction, &mut BorderColor),
    (Changed<Interaction>, With<DeselectButton>)
    >
) {
    if let Ok((interaction, mut border_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                player_loot.deselect_loot(&mut commands, selected_loot_ui, selected_loot_text, selected_loot_title);
                *border_color = Color::PURPLE.into();
            },
            Interaction::Hovered => {
                *border_color = Color::RED.into();
            }, 
            Interaction::None => {
                *border_color = Color::NONE.into();
            }
        }
    }
}

pub fn rotate_loot(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_loot: ResMut<PlayerLoot>,
    mut commands: Commands,
    selected_loot_ui: Query<Entity, With<SelectedLootIcon>>,
    selected_loot_text: Query<&mut Text, With<SelectedLootDescription>>,
    selected_loot_title: Query<&mut Text, (With<SelectedLootTitle>, Without<SelectedLootDescription>)>,
    asset_server: Res<AssetServer>
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        // "if" here because the player can press R without selecting anything
        if let Some(selected_loot) = player_loot.get_selected_loot_mut() {
            selected_loot.rotate_90_right();
            player_loot.redraw_selected_loot(&mut commands, selected_loot_ui, selected_loot_text, selected_loot_title, &asset_server);
        }
    }
}