use bevy::prelude::*;
use crate::game::ship_blocks::traits::{get_generic_info, get_generic_stats, get_generic_title, Description};
use crate::game::ship_blocks::turret::Turret;
use crate::ui::ship_edit_ui::components::{LootMenu, SelectedLootDescription, SelectedLootIcon, SelectedLootTitle};
use crate::game::ship_blocks::{components::Blocks, traits::Spawn};
use crate::ui::ship_edit_ui::{styles::*, components::LootUiBlock};

#[derive(Resource)]
pub struct PlayerLoot {
    pub looted_blocks: Vec<Blocks>,
}

impl Default for PlayerLoot {
    fn default() -> Self {
        PlayerLoot {
            looted_blocks: vec![],
        }
    }
}

impl PlayerLoot {
    pub fn remove_used_loot(
        &mut self,
        index: usize,
        commands: &mut Commands,
        selected_loot_icon: &Query<Entity, With<SelectedLootIcon>>,
        selected_loot_text: &mut Query<&mut Text, With<SelectedLootDescription>>,
        selected_loot_title: &mut Query<&mut Text, (With<SelectedLootTitle>, Without<SelectedLootDescription>)>,
    ) {
        self.looted_blocks.remove(index);
    }

    pub fn redraw_loot_ui(
        &self,
        commands: &mut Commands,
        loot_menu_query: &Query<Entity, With<LootMenu>>,
        asset_server: &Res<AssetServer>,
    ) {
        let loot_menu_entity = loot_menu_query.single();
        commands.entity(loot_menu_entity).despawn_descendants();
        commands.entity(loot_menu_entity).with_children(|parent| {
            self.spawn_loot_ui(parent, asset_server)
        });
    }

    pub fn spawn_loot_ui(
        &self,
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
    ) {
        for (i, loot) in self.looted_blocks.iter().enumerate() {
            parent.spawn((
                ButtonBundle {
                    style: mini_block(),
                    background_color: BLOCK_COLOR.into(),
                    ..default()
                },
                LootUiBlock {
                    index: i,
                    is_dragged: false
                }
            )).with_children(|parent| {
                loot.spawn_ui(parent, asset_server)
            });
        }
    }

    pub fn put_block_in_loot(
        &mut self,
        target_block: &Blocks,
    ) {
        self.looted_blocks.push(target_block.clone());
    }
}