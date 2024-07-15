use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use bevy::window::PrimaryWindow;
use bevy_kira_audio::{Audio, AudioControl};
use crate::game::player::components::{PlayerLoot, ShipLayout};
use crate::game::ship_blocks::components::Blocks;
use crate::game::ship_blocks::traits::{Rotate, Spawn};
use super::styles::{draggable, loot_selection_frame};
use super::{components::*, Selection};

#[derive(Default)]
pub struct TimeToDragg {
    time_pressed: f32,
}

pub fn interact_with_ui_blocks(
    mut commands: Commands,
    mut selection: ResMut<Selection>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut ship_layout: ResMut<ShipLayout>,
    mut player_loot: ResMut<PlayerLoot>,
    selected_loot_icon: Query<Entity, With<SelectedLootIcon>>,
    mut selected_loot_text: Query<&mut Text, With<SelectedLootDescription>>,
    mut selected_loot_title: Query<&mut Text, (With<SelectedLootTitle>, Without<SelectedLootDescription>)>,
    loot_menu_query: Query<Entity, With<LootMenu>>,
    mut button_query: Query<
        (&Interaction, &Children, &UiBlock, Entity, &Children),
        Changed<Interaction>
    >,
    block_sprite_query: Query<Entity, With<UISprite>>,
    mut frame_query: Query<&mut Visibility, With<BlockHoverFrame>>,
) {
    for (interaction, children, ui_block, block_entity, block_children) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if ship_layout.old_blocks_empty() {
                    ship_layout.old_blocks = ship_layout.blocks.clone();
                }
                for child in block_children {
                    if let Ok(sprite_entity) = block_sprite_query.get(*child) {
                        commands.entity(sprite_entity).despawn_recursive();
                    }
                }
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
                        audio.play(asset_server.load("audio/click.ogg")).with_volume(0.5);
                        *frame_visibility = Visibility::Visible;
                    }
                }
                println!("{} and {}", player_loot.is_loot_dragged, mouse.just_released(MouseButton::Left));
                if player_loot.is_loot_dragged && mouse.just_released(MouseButton::Left) {
                    if ship_layout.old_blocks_empty() {
                        ship_layout.old_blocks = ship_layout.blocks.clone();
                    }
                    for child in block_children {
                        if let Ok(sprite_entity) = block_sprite_query.get(*child) {
                            commands.entity(sprite_entity).despawn_recursive();
                        }
                    }
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
                }
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
    mut commands: Commands,
    mut player_loot: ResMut<PlayerLoot>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut time_to_dragg: Local<TimeToDragg>,
    mut button_query: Query<
    (&Interaction, Entity, &mut BorderColor, &mut LootUiBlock),
    Changed<Interaction>
    >,
    selected_loot_ui: Query<Entity, With<SelectedLootIcon>>,
    mut selected_loot_text: Query<&mut Text, With<SelectedLootDescription>>,
    mut selected_loot_title: Query<&mut Text, (With<SelectedLootTitle>, Without<SelectedLootDescription>)>,
    loot_menu_query: Query<Entity, With<LootMenu>>,
    old_frame_query: Query<Entity, With<LootSelectFrame>>,
    old_draggable_query: Query<Entity, With<Draggable>>
) {
    for (interaction, block_entity, mut border_color, mut loot_ui) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                player_loot.select_loot(loot_ui.index, &mut commands, &selected_loot_ui, &mut selected_loot_text, &mut selected_loot_title, &asset_server);
                spawn_loot_frame(block_entity, &mut commands, &asset_server, &old_frame_query);
                spawn_draggable(player_loot.get_selected_loot().unwrap(), &mut commands, &asset_server, &loot_menu_query, &old_draggable_query);
                loot_ui.is_dragged = true;
                //player_loot.redraw_selected_loot(&mut commands, selected_loot_ui, &asset_server);
                //*border_color = Color::PURPLE.into();
            },
            Interaction::Hovered => {
                *border_color = Color::RED.into();
                if loot_ui.is_dragged {
                    despawn_draggable(&mut commands, &old_draggable_query);
                    loot_ui.is_dragged = false;
                }
            },
            Interaction::None => {
                *border_color = Color::NONE.into();
                //player_loot.deselect_loot(&mut commands, &selected_loot_ui, &mut selected_loot_text, &mut selected_loot_title);
                //despawn_loot_frame(&mut commands, &old_frame_query);
                if loot_ui.is_dragged {
                    despawn_draggable(&mut commands, &old_draggable_query);
                    loot_ui.is_dragged = false;
                }
            }
        }
    }
}

pub fn spawn_loot_frame(
    target_entity: Entity,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    old_frame_query: &Query<Entity, With<LootSelectFrame>>
) {
    despawn_loot_frame(commands, old_frame_query);

    commands.entity(target_entity).with_children(|parent| {
        parent.spawn((
            ImageBundle {
                image: asset_server.load("sprites/select_frame.png").into(),
                style: loot_selection_frame(),
                z_index: ZIndex::Global(1),
                ..default()
            },
            LootSelectFrame {}
        ));
    });
}

pub fn despawn_loot_frame(
    commands: &mut Commands,
    old_frame_query: &Query<Entity, With<LootSelectFrame>>
) {
    for old_frame in old_frame_query.iter() {
        commands.entity(old_frame).despawn();
    }
}

pub fn spawn_draggable(
    target_block: &Blocks,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    loot_menu_query: &Query<Entity, With<LootMenu>>,
    old_draggable_query: &Query<Entity, With<Draggable>>,
) {
    despawn_draggable(commands, old_draggable_query);

    let loot_menu_entity = loot_menu_query.single();
    commands.entity(loot_menu_entity).with_children(|parent| {
        parent.spawn((
            NodeBundle {
                style: draggable(),
                ..default()
            },
            Draggable {}
        )).with_children(|parent| {
            target_block.spawn_ui(parent, &asset_server)
        });
    });
}

pub fn despawn_draggable(
    commands: &mut Commands,
    old_draggable_query: &Query<Entity, With<Draggable>>
) {
    for old_draggable in old_draggable_query.iter() {
        commands.entity(old_draggable).despawn_recursive();
    }
}

pub fn update_draggable(    
    time: Res<Time>,
    mut player_loot: ResMut<PlayerLoot>,
    mut time_to_dragg: Local<TimeToDragg>,
    window_query: Query<&Window, With<PrimaryWindow>>, 
    mut draggable_query: Query<&mut Style, With<Draggable>>,
    cursor_pos: Query<&RelativeCursorPosition, With<LootMenu>>
) {
    
    if let Ok(mut draggable_style) = draggable_query.get_single_mut() {
        if time_to_dragg.time_pressed < 0.3 {
            time_to_dragg.time_pressed += time.delta_seconds()
        } else {
            if !player_loot.is_loot_dragged {
                player_loot.is_loot_dragged = true;
            }
            let vmin = window_query.single().height();
            if let Some(cursor_pos) = cursor_pos.single().normalized {
                draggable_style.top = Val::Px(cursor_pos.y * vmin * 0.85085);
                draggable_style.left = Val::Px(cursor_pos.x * vmin * 0.4)
            }
        }
    } else if time_to_dragg.time_pressed != 0.0 {
        time_to_dragg.time_pressed = 0.0;
        player_loot.is_loot_dragged = false;
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
    mut selected_loot_text: Query<&mut Text, With<SelectedLootDescription>>,
    mut selected_loot_title: Query<&mut Text, (With<SelectedLootTitle>, Without<SelectedLootDescription>)>,
    asset_server: Res<AssetServer>
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        // "if" here because the player can press R without selecting anything
        if let Some(selected_loot) = player_loot.get_selected_loot_mut() {
            selected_loot.rotate_90_right();
            player_loot.redraw_selected_loot(&mut commands, &selected_loot_ui, &mut selected_loot_text, &mut selected_loot_title, &asset_server);
        }
    }
}