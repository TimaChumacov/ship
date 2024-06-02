use std::default;

use bevy::prelude::*;
use crate::game::ship_blocks::core::Core;
use crate::game::ship_blocks::harvester::Harvester;
use crate::game::ship_blocks::turret::Turret;
use crate::ui::ship_edit_ui::components::{LootMenu, SelectedLootUi};
use crate::game::ship_blocks::{components::Blocks, traits::Spawn};
use crate::ui::ship_edit_ui::{styles::*, components::LootUiBlock};

pub const PLAYER_SPEED: f32 = 100.0;

#[derive(Component)]
pub struct Player {}

#[derive(Resource)]
pub struct PlayerLoot {
    pub looted_blocks: Vec<Blocks>,
    pub selected_loot_index: Option<usize>,
}

impl Default for PlayerLoot {
    fn default() -> Self {
        PlayerLoot {
            looted_blocks: vec![],
            selected_loot_index: None,
        }
    }
}

impl PlayerLoot {
    pub fn get_selected_loot(&self) -> Option<&Blocks>{
        match self.selected_loot_index {
            Some(index) => {Some(&self.looted_blocks[index])}
            None => {None}
        }
    }

    pub fn get_selected_loot_mut(&mut self) -> Option<&mut Blocks>{
        match self.selected_loot_index {
            Some(index) => {Some(&mut self.looted_blocks[index])}
            None => {None}
        }
    }

    pub fn select_loot(
        &mut self, 
        target_index: usize,
        commands: &mut Commands,
        selected_loot_ui: Query<Entity, With<SelectedLootUi>>,
        asset_server: &Res<AssetServer>
    ) {
        self.selected_loot_index = Some(target_index);
        let selected_loot_ui_entity = selected_loot_ui.single();
        commands.entity(selected_loot_ui_entity).with_children(|parent| {
            self.looted_blocks[target_index].spawn_ui(parent, asset_server);
        });
    }

    pub fn deselect_loot(
        &mut self, 
        commands: &mut Commands,
        selected_loot_ui: Query<Entity, With<SelectedLootUi>>,
    ) {
        self.selected_loot_index = None;
        let selected_loot_ui_entity = selected_loot_ui.single();
        commands.entity(selected_loot_ui_entity).despawn_descendants();
    }

    pub fn redraw_selected_loot(
        &self,
        commands: &mut Commands,
        selected_loot_ui: Query<Entity, With<SelectedLootUi>>,
        asset_server: &Res<AssetServer>
    ) {
        let selected_loot_ui_entity = selected_loot_ui.single();
        commands.entity(selected_loot_ui_entity).despawn_descendants();
        commands.entity(selected_loot_ui_entity).with_children(|parent| {
            self.get_selected_loot().unwrap().spawn_ui(parent, asset_server)
        });
    }

    pub fn remove_used_loot(
        &mut self,
        commands: &mut Commands,
        selected_loot_ui: Query<Entity, With<SelectedLootUi>>,
    ) {
        self.looted_blocks.remove(self.selected_loot_index.unwrap());
        self.deselect_loot(commands, selected_loot_ui);
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
                    index: i
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

#[derive(Component)]
pub struct Ship {}

#[derive(Resource)]
pub struct ShipLayout {
    pub blocks: Vec<Vec<Option<Blocks>>>,
    pub old_blocks: Vec<Vec<Option<Blocks>>>,
}

impl Default for ShipLayout {
    fn default() -> Self {
        let mut blocks: Vec<Vec<Option<Blocks>>> = vec![vec![None; 5]; 5];
        blocks[1][0] = Some(Blocks::Core(Core::default()));
        blocks[1][2] = Some(Blocks::Turret(Turret::default()));
        blocks[3][4] = Some(Blocks::Harvester(Harvester::default()));
        ShipLayout {
            blocks: blocks,
            old_blocks: vec![vec![None]],
        }
    }
}

impl ShipLayout {
    pub fn spawn_ship(
        &self, 
        parent: &mut ChildBuilder, 
        asset_server: &Res<AssetServer>
    ) {
        for (a_usize, x) in self.blocks.iter().enumerate() {
            for (b_usize, y) in x.iter().enumerate() {
                if let Some(y) = y {
                    let (a, b) = (a_usize as f32, b_usize as f32);
                    y.spawn(
                        Vec3::new(a * 32.0 - 64.0, b * -32.0 + 64.0, 0.0), 
                        parent, 
                        asset_server
                    );
                }
            }
        }
    }

    pub fn despawn_ship(
        commands: &mut Commands,
        player_query: &Query<Entity, With<Player>>,
    ) {
        let player_entity = player_query.single();
        commands.entity(player_entity).despawn_descendants();
    }

    pub fn reset_ship(
        &self,
        mut commands: &mut Commands,
        player_query: &Query<Entity, With<Player>>,
        asset_server: &Res<AssetServer>
    ) {
        let player_entity = player_query.single();
        Self::despawn_ship(&mut commands, player_query);
        commands.entity(player_entity).with_children(|parent| {
            Self::spawn_ship(&self, parent, asset_server)
        });
    }

    pub fn update_ship(
        &mut self,
        mut commands: Commands,
        player_query: &Query<Entity, With<Player>>,
        asset_server: &Res<AssetServer>
    ) {
        let player_entity = player_query.single();
        for (a_usize, x) in self.blocks.iter().enumerate() {
            for (b_usize, y) in x.iter().enumerate() {
                if y != &self.old_blocks[a_usize][b_usize] {
                    if self.old_blocks[a_usize][b_usize].is_none() {
                        if let Some(y) = y {
                            let (a, b) = (a_usize as f32, b_usize as f32);
                            commands.entity(player_entity).with_children(|parent| {
                                y.spawn(
                                    Vec3::new(a * 32.0 - 64.0, b * -32.0 + 64.0, 0.0),
                                    parent, 
                                    asset_server
                                );
                            });
                        }
                    } else {
                        self.reset_ship(&mut commands, player_query, asset_server)
                    }
                }
            }
        }
        self.old_blocks = vec![vec![None]];
    }

    pub fn old_blocks_empty(&self) -> bool {
        self.old_blocks == vec![vec![None]]
    }
}