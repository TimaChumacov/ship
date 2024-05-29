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
    //mut loot_ui_blocks_query: Query<(Entity, &mut LootUiBlock)>,
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
                //if let Some(selected_block) = player_res.get_selected_loot() {
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
                        player_res.remove_used_loot();
                        player_res.reset_loot_ui(&mut commands, &loot_menu_query, &asset_server);
                    }
                } else {
                    // new loot ship_layout.blocks[ui_block.x][ui_block.y]
                    player_res.put_block_in_loot(&pressed_block.unwrap());
                    pressed_block = None;
                }
                ship_layout.blocks[ui_block.x][ui_block.y] = pressed_block;
                player_res.reset_loot_ui(&mut commands, &loot_menu_query, &asset_server)
                //}
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
) {
    if let Ok((interaction, mut border_color, small_ui_block)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                player_res.selected_loot_index = Some(small_ui_block.index);
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