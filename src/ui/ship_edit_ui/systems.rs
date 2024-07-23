use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use bevy::window::PrimaryWindow;
use bevy_kira_audio::{Audio, AudioControl};
use crate::game::player::components::ShipLayout;
use crate::game::player::player_loot::PlayerLoot;
use crate::game::ship_blocks::components::{Block, Blocks};
use crate::game::ship_blocks::traits::*;
use crate::ui::ship_edit_ui::*;
use super::styles::{draggable, loot_selection_frame, selection_frame};
use super::{components::*, BlockDisplay, DragAndDrop};

#[derive(Default)]
pub struct TimeToDragg {
    time_pressed: f32,
}

pub fn interact_with_ui_blocks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    // mouse: Res<ButtonInput<MouseButton>>,
    mut ship_layout: ResMut<ShipLayout>,
    mut block_display: ResMut<BlockDisplay>,
    //mut player_loot: ResMut<PlayerLoot>,
    mut drag_and_drop: ResMut<DragAndDrop>,
    // selected_loot_icon: Query<Entity, With<SelectedLootIcon>>,
    // mut selected_loot_text: Query<&mut Text, With<SelectedLootDescription>>,
    // mut selected_loot_title: Query<&mut Text, (With<SelectedLootTitle>, Without<SelectedLootDescription>)>,
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
                    // player_loot.deselect_loot(&mut commands, &selected_loot_icon, &mut selected_loot_text, &mut selected_loot_title); !
                    // player_loot.redraw_selected_loot(&pressed_block, &mut commands, &selected_loot_icon, &mut selected_loot_text, &mut selected_loot_title, &asset_server); !
                    block_display.block_to_display = ship_layout.blocks[ui_block.x][ui_block.y].clone();
                    block_display.block_index = Some(BlockIndex::Grid(ui_block.x, ui_block.y));
                    spawn_loot_frame( BlockOrigin::Grid, block_entity, &mut commands, &asset_server, &old_frame_query);
                    spawn_draggable(&pressed_block, &mut commands, &asset_server, &loot_menu_query, &old_draggable_query);
                    
                    ui_block.is_dragged = true;
                    drag_and_drop.is_block_pressed = true;
                    drag_and_drop.targeted_block = Some(block_entity);
                    drag_and_drop.block_origin = Some(BlockOrigin::Grid);
                    //ship_layout.dragged_block = Some(block_entity); !
                }
            },
            Interaction::Hovered => {
                if ui_block.is_dragged {
                    despawn_draggable(&mut commands, &old_draggable_query);
                    drag_and_drop.is_block_pressed = false;
                    ui_block.is_dragged = false;
                }

                drag_and_drop.hovered_block = Some(block_entity);
                //player_loot.hovered_block = Some(block_entity); !

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
                    drag_and_drop.is_block_pressed = false;
                    // Interaction and dragg update func try do stuff at the same time, so it's luck which one goes first
                }

                if drag_and_drop.hovered_block == Some(block_entity) {
                    drag_and_drop.hovered_block = None;
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
    mut drag_and_drop: ResMut<DragAndDrop>,
    mut block_display: ResMut<BlockDisplay>,
    selected_loot_icon: Query<Entity, With<SelectedLootIcon>>,
    mut selected_loot_text: Query<&mut Text, With<SelectedLootDescription>>,
    mut selected_loot_title: Query<&mut Text, (With<SelectedLootTitle>, Without<SelectedLootDescription>)>,
    loot_menu_query: Query<Entity, With<LootMenu>>,
    button_query: Query<(&UiBlock, &Children)>,
    block_sprite_query: Query<Entity, With<UISprite>>,
    old_frame_query: Query<Entity, With<LootSelectFrame>>,
) {
    if let Some(hovered_entity) = drag_and_drop.hovered_block {
        if let Ok((ui_hov_block, hov_block_children)) = button_query.get(hovered_entity) {
            if drag_and_drop.is_block_dragged && mouse.just_released(MouseButton::Left) &&
               hovered_entity != drag_and_drop.targeted_block.unwrap() {
                //println!("{}, {}, {}", player_loot.is_loot_dragged, ship_layout.dragged_block.is_some(), hovered_entity.index()); !
                // makes the slot in the grid visually empty (it's done even when they're already empty, just in case)
                for child in hov_block_children {
                    if let Ok(sprite_entity) = block_sprite_query.get(*child) {
                        commands.entity(sprite_entity).despawn_recursive();
                    }
                }
                // if the pressed block stored a block in the layout, we move that block to loot
                if let Some(pressed_block) = &mut ship_layout.blocks[ui_hov_block.x][ui_hov_block.y] {
                    player_loot.put_block_in_loot(pressed_block);
                    ship_layout.blocks[ui_hov_block.x][ui_hov_block.y] = None;
                }

                let mut placed_block: Option<Blocks> = None; // var to store the block that is gonna replace whatever block you hovered at the start
                match drag_and_drop.block_origin.clone().unwrap() {
                    // --- the logic for moving loot inside the grid ---
                    BlockOrigin::Grid => {
                        let (ui_block, block_children) = button_query.get(drag_and_drop.targeted_block.unwrap()).unwrap();
                        for child in block_children {
                            if let Ok(sprite_entity) = block_sprite_query.get(*child) {
                                commands.entity(sprite_entity).despawn_recursive();
                            }
                        }
                        placed_block = ship_layout.blocks[ui_block.x][ui_block.y].clone();
                        commands.entity(hovered_entity).with_children(|parent| {
                            placed_block.as_ref().unwrap().spawn_ui(parent, &asset_server)
                        });
                        drag_and_drop.targeted_block = None;
                        ship_layout.blocks[ui_block.x][ui_block.y] = None;
                        block_display.block_to_display = placed_block.clone();
                        // player_loot.deselect_loot(&mut commands, &selected_loot_icon, &mut selected_loot_text, &mut selected_loot_title); !
                        // player_loot.redraw_selected_loot(placed_block.as_ref().unwrap(), &mut commands, &selected_loot_icon, &mut selected_loot_text, &mut selected_loot_title, &asset_server);
                        spawn_loot_frame(BlockOrigin::Grid, hovered_entity, &mut commands, &asset_server, &old_frame_query)
                    },
                    // --- the logic for dragging loot from loot inventory ---
                    BlockOrigin::Loot => {
                        placed_block = Some(player_loot.looted_blocks[block_display.block_index.unwrap().get_loot_index().unwrap()].clone());
                        commands.entity(hovered_entity).with_children(|parent| {
                            placed_block.as_ref().unwrap().spawn_ui(parent, &asset_server)
                        });
                        let index = block_display.block_index.as_ref().unwrap().get_loot_index().unwrap();
                        player_loot.remove_used_loot(index, &mut commands, &selected_loot_icon, &mut selected_loot_text, &mut selected_loot_title);
                        block_display.block_to_display = placed_block.clone();
                        // player_loot.redraw_loot_ui(&mut commands, &loot_menu_query, &asset_server);
                        // player_loot.redraw_selected_loot(placed_block.as_ref().unwrap(), &mut commands, &selected_loot_icon, &mut selected_loot_text, &mut selected_loot_title, &asset_server)
                    }
                }
                // saving the newly placed block in the grid (and redrawing ui)
                ship_layout.blocks[ui_hov_block.x][ui_hov_block.y] = placed_block;
                player_loot.redraw_loot_ui(&mut commands, &loot_menu_query, &asset_server);

                if ship_layout.old_blocks_empty() {
                    ship_layout.old_blocks = ship_layout.blocks.clone();
                }
            }
        }
    }
}

pub fn check_loot_dropped_into_loot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut ship_layout: ResMut<ShipLayout>,
    mut player_loot: ResMut<PlayerLoot>,
    mut drag_and_drop: ResMut<DragAndDrop>,
    mut block_display: ResMut<BlockDisplay>,
    // selected_loot_icon: Query<Entity, With<SelectedLootIcon>>,
    // mut selected_loot_text: Query<&mut Text, With<SelectedLootDescription>>,
    // mut selected_loot_title: Query<&mut Text, (With<SelectedLootTitle>, Without<SelectedLootDescription>)>,
    loot_menu_query: Query<Entity, With<LootMenu>>,
    button_query: Query<&Children>,
    block_sprite_query: Query<Entity, With<UISprite>>,
    old_frame_query: Query<Entity, With<LootSelectFrame>>,
    old_draggable_query: Query<Entity, With<Draggable>>
) {
    if drag_and_drop.block_origin == Some(BlockOrigin::Grid) && 
       drag_and_drop.hovered_into_loot && 
       mouse.just_released(MouseButton::Left) {
        let (x, y) = block_display.block_index.unwrap().get_block_xy().unwrap();
        let target_block = ship_layout.blocks[x][y].clone().unwrap();
        player_loot.put_block_in_loot(&target_block);
        player_loot.redraw_loot_ui(&mut commands, &loot_menu_query, &asset_server);
        // clean up
        let block_children = button_query.get(drag_and_drop.targeted_block.unwrap()).unwrap();
        for child in block_children {
            if let Ok(sprite_entity) = block_sprite_query.get(*child) {
                commands.entity(sprite_entity).despawn_recursive();
            }
        }
        ship_layout.blocks[x][y] = None;
        block_display.block_to_display = None;
        despawn_draggable(&mut commands, &old_draggable_query);
        despawn_loot_frame(&mut commands, &old_frame_query);
        *drag_and_drop = DragAndDrop::default();
    }
}

// Function check_drag_and_dropped_into_loot with if mouse released && drag_and_drop.hovered_over_loot 

pub fn interact_with_ui_loot(
    mut commands: Commands,
    player_loot: ResMut<PlayerLoot>,
    mut drag_and_drop: ResMut<DragAndDrop>,
    mut block_display: ResMut<BlockDisplay>,
    asset_server: Res<AssetServer>,
    mut button_query: Query<
    (&Interaction, Entity, &mut BorderColor, &mut LootUiBlock),
    Changed<Interaction>
    >,
    loot_menu_query: Query<Entity, With<LootMenu>>,
    old_frame_query: Query<Entity, With<LootSelectFrame>>,
    old_draggable_query: Query<Entity, With<Draggable>>
) {
    for (interaction, block_entity, mut border_color, mut loot_ui) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                let loot_block = &player_loot.looted_blocks[loot_ui.index];
                block_display.block_to_display = Some(loot_block.clone());
                block_display.block_index = Some(BlockIndex::Loot(loot_ui.index));
                spawn_loot_frame( BlockOrigin::Loot, block_entity, &mut commands, &asset_server, &old_frame_query);
                spawn_draggable(&loot_block, &mut commands, &asset_server, &loot_menu_query, &old_draggable_query);
                loot_ui.is_dragged = true;
                drag_and_drop.block_origin = Some(BlockOrigin::Loot);
                drag_and_drop.targeted_block = Some(block_entity);
                drag_and_drop.is_block_pressed = true;
            },
            Interaction::Hovered => {
                *border_color = Color::srgb(1.0, 0.0, 0.0).into();
                if loot_ui.is_dragged {
                    despawn_draggable(&mut commands, &old_draggable_query);
                    loot_ui.is_dragged = false;
                    drag_and_drop.is_block_pressed = false;
                }
            },
            Interaction::None => {
                *border_color = Color::NONE.into();
                if loot_ui.is_dragged {
                    despawn_draggable(&mut commands, &old_draggable_query);
                    loot_ui.is_dragged = false;
                    drag_and_drop.is_block_pressed = false;
                }
            }
        }
    }
}

pub fn spawn_loot_frame(
    target_type: BlockOrigin,
    target_entity: Entity,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    old_frame_query: &Query<Entity, With<LootSelectFrame>>
) {
    despawn_loot_frame(commands, old_frame_query);

    let frame_style: Style;
    match target_type {
        BlockOrigin::Grid => {frame_style = selection_frame()},
        BlockOrigin::Loot => {frame_style = loot_selection_frame()},
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
    mut drag_and_drop: ResMut<DragAndDrop>,
    mut time_to_dragg: Local<TimeToDragg>,
    window_query: Query<&Window, With<PrimaryWindow>>, 
    mut draggable_query: Query<&mut Style, With<Draggable>>,
    cursor_pos: Query<&RelativeCursorPosition, With<LootMenu>>
) {
    
    if let Ok(mut draggable_style) = draggable_query.get_single_mut() {
        if drag_and_drop.is_block_pressed {
            if time_to_dragg.time_pressed < 0.15 {
                time_to_dragg.time_pressed += time.delta_seconds()
            } else {
                if !drag_and_drop.is_block_dragged {
                    drag_and_drop.is_block_dragged = true;
                }
                let vmin = window_query.single().height();
                if let Some(cursor_pos) = cursor_pos.single().normalized {
                    draggable_style.top = Val::Px(cursor_pos.y * vmin * 0.85085);
                    draggable_style.left = Val::Px(cursor_pos.x * vmin * 0.4);
                    if 0.0 < cursor_pos.x && cursor_pos.x < 1.0 &&
                       0.0 < cursor_pos.y && cursor_pos.y < 1.0 {
                        drag_and_drop.hovered_into_loot = true
                    } else if drag_and_drop.hovered_into_loot {
                        drag_and_drop.hovered_into_loot = false
                    }
                }
            }
        }
    } else if time_to_dragg.time_pressed != 0.0 {
        time_to_dragg.time_pressed = 0.0;
        drag_and_drop.is_block_dragged = false;
    }
}

pub fn update_draggable_color(
    drag_and_drop: Res<DragAndDrop>,
    draggable_children: Query<&Children, With<Draggable>>,
    mut ui_image_query: Query<&mut UiImage, With<UISprite>>
) {
    if drag_and_drop.is_block_dragged {
        if let Ok(children) = draggable_children.get_single() {
            for child in children {
                if let Ok(mut ui_image) = ui_image_query.get_mut(*child) {
                    if drag_and_drop.hovered_block.is_some() {
                        ui_image.color = Color::srgb(0.5, 1.0, 0.5);
                    } else {
                        ui_image.color = Color::srgb(1.0, 0.5, 0.5);
                    }
                    if drag_and_drop.hovered_into_loot && drag_and_drop.block_origin == Some(BlockOrigin::Grid) {
                        ui_image.color = Color::srgb(0.6, 0.6, 1.0);
                    }
                }
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
                //player_loot.deselect_loot(&mut commands, &selected_loot_ui, &mut selected_loot_text, &mut selected_loot_title);
                *border_color = Color::srgb(1.0, 0.0, 1.0).into();
            },
            Interaction::Hovered => {
                *border_color = Color::srgb(1.0, 0.0, 0.0).into();
            }, 
            Interaction::None => {
                *border_color = Color::NONE.into();
            }
        }
    }
}

pub fn rotate_loot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ship_layout: ResMut<ShipLayout>,
    mut player_loot: ResMut<PlayerLoot>,
    mut block_display: ResMut<BlockDisplay>,
    drag_and_drop: Res<DragAndDrop>,
    //ui_block_query: Query<&mut Blo>,
    selected_loot_ui: Query<Entity, With<SelectedLootIcon>>,
    mut selected_loot_text: Query<&mut Text, With<SelectedLootDescription>>,
    mut selected_loot_title: Query<&mut Text, (With<SelectedLootTitle>, Without<SelectedLootDescription>)>,
    loot_menu_query: Query<Entity, With<LootMenu>>,
    children_query: Query<&Children>,
    block_sprite_query: Query<Entity, With<UISprite>>,
    old_draggable_query: Query<Entity, With<Draggable>>
) {
    if keyboard_input.just_pressed(KeyCode::KeyQ) ||
       keyboard_input.just_pressed(KeyCode::KeyE) {
        if let Some(block_index) = block_display.block_index {
            let block_to_rotate: &mut Blocks;
            match block_index {
                BlockIndex::Grid(x, y) => {
                    block_to_rotate = ship_layout.blocks[x][y].as_mut().unwrap();
                },
                BlockIndex::Loot(index) => {
                    block_to_rotate = &mut player_loot.looted_blocks[index];
                }
            }
            if keyboard_input.just_pressed(KeyCode::KeyQ) {
                block_to_rotate.rotate_90(RotDirection::Left);
            }
            if keyboard_input.just_pressed(KeyCode::KeyE) {
                block_to_rotate.rotate_90(RotDirection::Right);
            }
            if drag_and_drop.is_block_dragged {
                spawn_draggable(&block_to_rotate, &mut commands, &asset_server, &loot_menu_query, &old_draggable_query);
            }
            let target_entity = drag_and_drop.targeted_block.unwrap();
            let children = children_query.get(target_entity).unwrap();
            for child in children {
                if let Ok(child_entity) = block_sprite_query.get(*child) {
                    commands.entity(child_entity).despawn_recursive();
                    commands.entity(target_entity).with_children(|parent| {
                        block_to_rotate.spawn_ui(parent, &asset_server);
                    });
                }
            }
        }
    }
}

pub fn update_block_display(
    mut commands: Commands,
    block_display: Res<BlockDisplay>,
    selected_loot_icon: Query<Entity, With<SelectedLootIcon>>,
    mut selected_loot_text: Query<&mut Text, With<SelectedLootDescription>>,
    mut selected_loot_title: Query<&mut Text, (With<SelectedLootTitle>, Without<SelectedLootDescription>)>,
    asset_server: Res<AssetServer>
) {
    if block_display.is_changed() {
        if let Some(target_block) = &block_display.block_to_display {
            let selected_loot_icon_entity = selected_loot_icon.single();
            commands.entity(selected_loot_icon_entity).despawn_descendants();
            commands.entity(selected_loot_icon_entity).with_children(|parent| {
                target_block.spawn_ui(parent, &asset_server)
            });
            let mut selected_loot_text = selected_loot_text.single_mut();
            selected_loot_text.sections[0].value = target_block.get_stats();
            selected_loot_text.sections[1].value = target_block.get_info();
            let mut selected_loot_title = selected_loot_title.single_mut();
            selected_loot_title.sections[0].value = target_block.get_title();
        } else {
            if let Ok(selected_loot_icon_entity) = selected_loot_icon.get_single() {
                commands.entity(selected_loot_icon_entity).despawn_descendants();
                let mut selected_loot_text = selected_loot_text.single_mut();
                selected_loot_text.sections[0].value = get_generic_stats();
                selected_loot_text.sections[1].value = get_generic_info();
                let mut selected_loot_title = selected_loot_title.single_mut();
                selected_loot_title.sections[0].value = get_generic_title();
            }
        }
    }
} 