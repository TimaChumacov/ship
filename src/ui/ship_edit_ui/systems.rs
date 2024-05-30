use bevy::prelude::*;
use bevy::transform::commands;

use crate::game::player::components::{PlayerResource, ShipLayout};
use crate::game::ship_blocks::components::Blocks;
use super::components::*;

pub fn interact_with_ui_blocks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ship_layout: ResMut<ShipLayout>,
    mut player_res: ResMut<PlayerResource>,
    selected_loot_ui: Query<Entity, With<SelectedLootUi>>,
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
                    if let Some(selected_block) = player_res.get_selected_loot() {
                        pressed_block = Some(selected_block.clone());
                        commands.entity(block_entity).with_children(|parent| {
                            selected_block.spawn_ui(parent, &asset_server)
                        });
                        player_res.remove_used_loot(&mut commands, selected_loot_ui);
                        player_res.reset_loot_ui(&mut commands, &loot_menu_query, &asset_server);
                    }
                } else {
                    player_res.put_block_in_loot(&pressed_block.unwrap());
                    pressed_block = None;
                }
                ship_layout.blocks[ui_block.x][ui_block.y] = pressed_block;
                player_res.reset_loot_ui(&mut commands, &loot_menu_query, &asset_server)
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

pub fn interact_with_ui_loot(
    mut player_res: ResMut<PlayerResource>,
    mut button_query: Query<
    (&Interaction, &mut BorderColor, &LootUiBlock),
    Changed<Interaction>
    >,
    mut commands: Commands,
    selected_loot_ui: Query<Entity, With<SelectedLootUi>>,
    asset_server: Res<AssetServer>
) {
    if let Ok((interaction, mut border_color, small_ui_block)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                player_res.select_loot(small_ui_block.index, &mut commands, selected_loot_ui, &asset_server);
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
    mut player_res: ResMut<PlayerResource>,
    selected_loot_ui: Query<Entity, With<SelectedLootUi>>,
    mut button_query: Query<
    (&Interaction, &mut BorderColor),
    (Changed<Interaction>, With<DeselectButton>)
    >
) {
    if let Ok((interaction, mut border_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                player_res.deselect_loot(&mut commands, selected_loot_ui);
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