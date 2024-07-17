use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use bevy::window::PrimaryWindow;
use bevy_kira_audio::{Audio, AudioControl};
use crate::game::player::components::{PlayerLoot, ShipLayout};
use crate::game::ship_blocks::components::{Block, Blocks};
use crate::game::ship_blocks::traits::{Rotate, Spawn};
use super::styles::{draggable, loot_selection_frame, selection_frame};
use super::{components::*, Selection};

#[derive(Default)]
pub struct TimeToDragg {
    time_pressed: f32,
}

pub fn interact_with_ui_blocks(
    mut commands: Commands,
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
        (&Interaction, &Children, &mut UiBlock, Entity, &Children),
        Changed<Interaction>
    >,
    mut frame_query: Query<&mut Visibility, With<BlockHoverFrame>>,
    old_frame_query: Query<Entity, With<LootSelectFrame>>,
    old_draggable_query: Query<Entity, With<Draggable>>,
    block_sprite_query: Query<Entity, With<UISprite>>,
) {
    for (interaction, children, mut ui_block, block_entity, block_children) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if let Some(pressed_block) = ship_layout.blocks[ui_block.x][ui_block.y].clone() {
                    player_loot.deselect_loot(&mut commands, &selected_loot_icon, &mut selected_loot_text, &mut selected_loot_title);
                    player_loot.redraw_selected_loot(&pressed_block, &mut commands, &selected_loot_icon, &mut selected_loot_text, &mut selected_loot_title, &asset_server);
                    spawn_loot_frame( TargetType::Block, block_entity, &mut commands, &asset_server, &old_frame_query);
                    spawn_draggable(&pressed_block, &mut commands, &asset_server, &loot_menu_query, &old_draggable_query);
                    ui_block.is_dragged = true;
                    ship_layout.dragged_block = Some(block_entity);
                    println!("DRAGGED BLOCK IS ID-{}", block_entity.index());
                }
            },
            Interaction::Hovered => {
                if ui_block.is_dragged {
                    despawn_draggable(&mut commands, &old_draggable_query);
                    ui_block.is_dragged = false;
                    ship_layout.dragged_block = None;
                    println!("HOVER BLOCK ID-{}: DRAGGED BLOCK IS NONE", block_entity.index());
                }

                player_loot.hovered_block = Some(block_entity);

                for child in children {
                    if let Ok(mut frame_visibility) = frame_query.get_mut(*child) {
                        audio.play(asset_server.load("audio/click.ogg")).with_volume(0.5);
                        *frame_visibility = Visibility::Visible;
                    }
                }
            },
            Interaction::None => {
                if ui_block.is_dragged {
                    despawn_draggable(&mut commands, &old_draggable_query);
                    ui_block.is_dragged = false;
                    ship_layout.dragged_block = None;
                    println!("NONE BLOCK ID-{}: DRAGGED BLOCK IS NONE", block_entity.index());
                    // Interaction and dragg update func try do stuff at the same time, so it's luck which one goes first
                }

                if player_loot.hovered_block == Some(block_entity) {
                    player_loot.hovered_block = None;
                }

                for child in children {
                    if let Ok(mut frame_visibility) = frame_query.get_mut(*child) {
                        *frame_visibility = Visibility::Hidden;
                    }
                }
            }
        }
    }
}

pub fn check_dragg_and_dropped(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut ship_layout: ResMut<ShipLayout>,
    mut player_loot: ResMut<PlayerLoot>,
    selected_loot_icon: Query<Entity, With<SelectedLootIcon>>,
    mut selected_loot_text: Query<&mut Text, With<SelectedLootDescription>>,
    mut selected_loot_title: Query<&mut Text, (With<SelectedLootTitle>, Without<SelectedLootDescription>)>,
    loot_menu_query: Query<Entity, With<LootMenu>>,
    button_query: Query<(&UiBlock, &Children)>,
    block_sprite_query: Query<Entity, With<UISprite>>,
    old_frame_query: Query<Entity, With<LootSelectFrame>>,
) {
    if let Some(hovered_entity) = player_loot.hovered_block {
        if let Ok((ui_hov_block, hov_block_children)) = button_query.get(hovered_entity) {
            if (player_loot.is_loot_dragged) && 
                mouse.just_released(MouseButton::Left) {
                println!("{}, {}, {}", player_loot.is_loot_dragged, ship_layout.dragged_block.is_some(), hovered_entity.index());
                for child in hov_block_children {
                    if let Ok(sprite_entity) = block_sprite_query.get(*child) {
                        println!("blasphemous child perished");
                        commands.entity(sprite_entity).despawn_recursive();
                    }
                }
                let mut pressed_block = ship_layout.blocks[ui_hov_block.x][ui_hov_block.y].clone();
                if pressed_block.is_some() {
                    player_loot.put_block_in_loot(&pressed_block.unwrap());
                    pressed_block = None;
                }
                if let Some(selected_block) = player_loot.get_selected_loot() {
                    pressed_block = Some(selected_block.clone());
                    commands.entity(hovered_entity).with_children(|parent| {
                        selected_block.spawn_ui(parent, &asset_server)
                    });
                    player_loot.remove_used_loot(&mut commands, &selected_loot_icon, &mut selected_loot_text, &mut selected_loot_title);
                    player_loot.redraw_loot_ui(&mut commands, &loot_menu_query, &asset_server);
                }
                if let Some(block_entity) = ship_layout.dragged_block {
                    let (ui_block, block_children) = button_query.get(block_entity).unwrap();
                    for child in block_children {
                        if let Ok(sprite_entity) = block_sprite_query.get(*child) {
                            println!("oh bayle, the blasphemous child perished!");
                            commands.entity(sprite_entity).despawn_recursive();
                        }
                    }
                    pressed_block = ship_layout.blocks[ui_block.x][ui_block.y].clone();
                    commands.entity(hovered_entity).with_children(|parent| {
                        pressed_block.as_ref().unwrap().spawn_ui(parent, &asset_server)
                    });
                    ship_layout.dragged_block = None;
                    ship_layout.blocks[ui_block.x][ui_block.y] = None;
                    player_loot.deselect_loot(&mut commands, &selected_loot_icon, &mut selected_loot_text, &mut selected_loot_title);
                    spawn_loot_frame(TargetType::Block, hovered_entity, &mut commands, &asset_server, &old_frame_query)
                }
                ship_layout.blocks[ui_hov_block.x][ui_hov_block.y] = pressed_block;
                player_loot.redraw_loot_ui(&mut commands, &loot_menu_query, &asset_server);

                if ship_layout.old_blocks_empty() {
                    println!("old ones were denied their silence");
                    ship_layout.old_blocks = ship_layout.blocks.clone();
                }
            }
        }
    }
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
                spawn_loot_frame( TargetType::Loot, block_entity, &mut commands, &asset_server, &old_frame_query);
                spawn_draggable(player_loot.get_selected_loot().unwrap(), &mut commands, &asset_server, &loot_menu_query, &old_draggable_query);
                loot_ui.is_dragged = true;
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
                if loot_ui.is_dragged {
                    despawn_draggable(&mut commands, &old_draggable_query);
                    loot_ui.is_dragged = false;
                }
            }
        }
    }
}

pub enum TargetType {
    Block,
    Loot
}

pub fn spawn_loot_frame(
    target_type: TargetType,
    target_entity: Entity,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    old_frame_query: &Query<Entity, With<LootSelectFrame>>
) {
    despawn_loot_frame(commands, old_frame_query);

    let frame_style: Style;
    match target_type {
        TargetType::Block => {frame_style = selection_frame()},
        TargetType::Loot => {frame_style = loot_selection_frame()},
    }
    commands.entity(target_entity).with_children(|parent| {
        parent.spawn((
            ImageBundle {
                image: asset_server.load("sprites/select_frame.png").into(),
                style: frame_style,
                z_index: ZIndex::Global(3),
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
        if time_to_dragg.time_pressed < 0.15 {
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
            player_loot.redraw_selected_loot( player_loot.get_selected_loot().unwrap(), &mut commands, &selected_loot_ui, &mut selected_loot_text, &mut selected_loot_title, &asset_server);
        }
    }
}