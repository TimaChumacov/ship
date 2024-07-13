use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
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
    mut selected_loot_text: Query<&mut Text, With<SelectedLootDescription>>,
    mut selected_loot_title: Query<&mut Text, (With<SelectedLootTitle>, Without<SelectedLootDescription>)>,
    loot_menu_query: Query<Entity, With<LootMenu>>,
    mut button_query: Query<
        (&Interaction, &Children, &UiBlock, Entity),
        Changed<Interaction>
    >,
    mut frame_query: Query<&mut Visibility, With<BlockHoverFrame>>
) {
    for (interaction, children, ui_block, block_entity) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
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
                        player_loot.remove_used_loot(&mut commands, &selected_loot_icon, &mut selected_loot_text, &mut selected_loot_title);
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
                for child in children {
                    if let Ok(mut frame_visibility) = frame_query.get_mut(*child) {
                        *frame_visibility = Visibility::Visible;
                    }
                }
                //*border_color = Color::RED.into();
                //selection.new_hovered_block = Some(block_entity);
                //selection.block_hover_frame = None;
            },
            Interaction::None => {
                for child in children {
                    if let Ok(mut frame_visibility) = frame_query.get_mut(*child) {
                        *frame_visibility = Visibility::Hidden;
                    }
                }
                //*border_color = Color::NONE.into();
                // if let Some(old_frame) = selection.block_hover_frame {
                //     selection.block_hover_frame = None;
                //     commands.entity(old_frame).despawn();
                // }
            }
        }
    }
}

pub fn animate_selection(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    //relative_cursor_pos: Query<&RelativeCursorPosition>,
    mut selection: ResMut<Selection>,
    grid_query: Query<Entity, With<Gridmenu>>,
    mut block_hover_frame_query: Query<&RelativeCursorPosition, With<BlockHoverFrame>>,
) {
    // It's much easier, make the frames invisible children and turn them on with hover and stuff
    // the selection frames can also be children i guess


    // // if a block is hovered...
    // if let Some(hovered_entity) = selection.new_hovered_block {
    //     // and if the hovered block is new... 
    //     if selection.currently_hovered_block != selection.new_hovered_block {
    //         // delete the old frame if it exists...
    //         if selection.block_hover_frame.is_some() {
    //             commands.entity(selection.block_hover_frame.unwrap()).despawn();
    //         }
    //         // spawn a new one..
    //         commands.entity(hovered_entity).with_children(|parent| {
    //             // store it in a resource...
    //             selection.block_hover_frame = Some(
    //                 parent.spawn((
    //                     ImageBundle {
    //                         image: asset_server.load("sprites/hover_frame.png").into(),
    //                         style: selection_frame(),
    //                         ..default()
    //                     },
    //                     BlockHoverFrame {}
    //                 ))
    //                 .insert(RelativeCursorPosition::default())
    //                 .id()
    //             )
    //         });
    //         // now the currently hovered block is the one that was new
    //         selection.currently_hovered_block = selection.new_hovered_block;
    //     }   
    // }
    // if let Ok(cursor_pos) = block_hover_frame_query.get_single_mut() {
    //     if let Some(pos) = cursor_pos.normalized {
    //         if selection.block_hover_frame.is_some() &&
    //             0.0 > pos.y && pos.y > 1.0 &&
    //             0.0 > pos.x && pos.x > 1.0
    //         {
    //             commands.entity(selection.block_hover_frame.unwrap()).despawn();
    //             selection.block_hover_frame = None;
    //         }
    //     }
    //      // block_hover_frame_style.top = Val::Px(relative_cursor_pos.single().normalized.unwrap().y) * 100.0;
    //      // block_hover_frame_style.left = Val::Px(relative_cursor_pos.single().normalized.unwrap().x) * 100.0;
    // }   
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
    mut selected_loot_text: Query<&mut Text, With<SelectedLootDescription>>,
    mut selected_loot_title: Query<&mut Text, (With<SelectedLootTitle>, Without<SelectedLootDescription>)>,
    mut button_query: Query<
    (&Interaction, &mut BorderColor),
    (Changed<Interaction>, With<DeselectButton>)
    >
) {
    if let Ok((interaction, mut border_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                player_loot.deselect_loot(&mut commands, &selected_loot_ui, &mut selected_loot_text, &mut selected_loot_title);
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