use bevy::prelude::*;
use bevy::transform::commands;

use crate::game::player::components::ShipLayout;
use crate::game::ship_blocks::components::Blocks;
use super::components::{SmallUiBlock, UiBlock};

pub fn interact_with_ui_blocks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ship_layout: ResMut<ShipLayout>,
    mut button_query: Query<
    (&Interaction, &mut BorderColor, &UiBlock, Entity),
    Changed<Interaction>
    >,
) {
    if let Ok((interaction, mut border_color, ui_block, button_entity)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *border_color = Color::PURPLE.into();
                if ship_layout.old_blocks_empty() {
                    ship_layout.old_blocks = ship_layout.blocks.clone();
                }
                commands.entity(button_entity).despawn_descendants();
                if ship_layout.blocks[ui_block.x][ui_block.y] == None {
                    ship_layout.blocks[ui_block.x][ui_block.y] = Some(Blocks::Core);
                    commands.entity(button_entity).with_children(|parent| {
                        Blocks::Core.spawn_ui(parent, &asset_server)
                    });
                } else {
                    ship_layout.blocks[ui_block.x][ui_block.y] = None;
                }
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
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ship_layout: ResMut<ShipLayout>,
    mut button_query: Query<
    (&Interaction, &mut BorderColor, &SmallUiBlock, Entity),
    Changed<Interaction>
    >,
) {
    if let Ok((interaction, mut border_color, ui_block, button_entity)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
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