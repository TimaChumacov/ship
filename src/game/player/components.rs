use bevy::prelude::*;
use crate::game::ship_blocks::core::Core;
use crate::game::ship_blocks::harvester::Harvester;
use crate::game::ship_blocks::traits::{get_generic_info, get_generic_stats, get_generic_title, Description};
use crate::game::ship_blocks::turret::Turret;
use crate::ui::ship_edit_ui::components::{LootMenu, SelectedLootDescription, SelectedLootIcon, SelectedLootTitle};
use crate::game::ship_blocks::{components::Blocks, traits::Spawn};
use crate::ui::ship_edit_ui::{styles::*, components::LootUiBlock};

pub const PLAYER_SPEED: f32 = 100.0;

#[derive(Component)]
pub struct Player {}

impl Player {
    pub fn get_rotation(transform: &Transform) -> f32 {
        transform.rotation.to_euler(EulerRot::ZXY).0.to_degrees()
    }
}

#[derive(Resource)]
pub struct PlayerLoot {
    pub looted_blocks: Vec<Blocks>,
    pub selected_loot_index: Option<usize>,
}

impl Default for PlayerLoot {
    fn default() -> Self {
        PlayerLoot {
            looted_blocks: vec![Blocks::Turret(Turret::default()); 35],
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
        selected_loot_icon: Query<Entity, With<SelectedLootIcon>>,
        selected_loot_text: Query<&mut Text, With<SelectedLootDescription>>,
        selected_loot_title: Query<&mut Text, (With<SelectedLootTitle>, Without<SelectedLootDescription>)>,
        asset_server: &Res<AssetServer>
    ) {
        self.selected_loot_index = Some(target_index);
        self.redraw_selected_loot(commands, selected_loot_icon, selected_loot_text, selected_loot_title, asset_server);
    }

    pub fn deselect_loot(
        &mut self, 
        commands: &mut Commands,
        selected_loot_ui: &Query<Entity, With<SelectedLootIcon>>,
        selected_loot_text: &mut Query<&mut Text, With<SelectedLootDescription>>,
        selected_loot_title: &mut Query<&mut Text, (With<SelectedLootTitle>, Without<SelectedLootDescription>)>,
    ) {
        self.selected_loot_index = None;
        let selected_loot_ui_entity = selected_loot_ui.single();
        commands.entity(selected_loot_ui_entity).despawn_descendants();
        let mut selected_loot_text = selected_loot_text.single_mut();
        selected_loot_text.sections[0].value = get_generic_stats();
        selected_loot_text.sections[1].value = get_generic_info();
        let mut selected_loot_title = selected_loot_title.single_mut();
        selected_loot_title.sections[0].value = get_generic_title();
    }

    pub fn redraw_selected_loot(
        &self,
        commands: &mut Commands,
        selected_loot_ui: Query<Entity, With<SelectedLootIcon>>,
        mut selected_loot_text: Query<&mut Text, With<SelectedLootDescription>>,
        mut selected_loot_title: Query<&mut Text, (With<SelectedLootTitle>, Without<SelectedLootDescription>)>,
        asset_server: &Res<AssetServer>
    ) {
        let selected_loot_ui_entity = selected_loot_ui.single();
        commands.entity(selected_loot_ui_entity).despawn_descendants();
        commands.entity(selected_loot_ui_entity).with_children(|parent| {
            self.get_selected_loot().unwrap().spawn_ui(parent, asset_server)
        });
        let mut selected_loot_text = selected_loot_text.single_mut();
        selected_loot_text.sections[0].value = self.get_selected_loot().unwrap().get_stats();
        selected_loot_text.sections[1].value = self.get_selected_loot().unwrap().get_info();
        let mut selected_loot_title = selected_loot_title.single_mut();
        selected_loot_title.sections[0].value = self.get_selected_loot().unwrap().get_title();
    }

    pub fn remove_used_loot(
        &mut self,
        commands: &mut Commands,
        selected_loot_icon: &Query<Entity, With<SelectedLootIcon>>,
        selected_loot_text: &mut Query<&mut Text, With<SelectedLootDescription>>,
        selected_loot_title: &mut Query<&mut Text, (With<SelectedLootTitle>, Without<SelectedLootDescription>)>,
    ) {
        self.looted_blocks.remove(self.selected_loot_index.unwrap());
        self.deselect_loot(commands, selected_loot_icon, selected_loot_text, selected_loot_title);
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

#[derive(Resource)]
pub struct ShipLayout {
    pub blocks: Vec<Vec<Option<Blocks>>>,
    pub old_blocks: Vec<Vec<Option<Blocks>>>,
}

impl Default for ShipLayout {
    fn default() -> Self {
        let mut blocks: Vec<Vec<Option<Blocks>>> = vec![vec![None; 5]; 5];
        blocks[2][2] = Some(Blocks::Core(Core::default()));
        blocks[2][1] = Some(Blocks::Turret(Turret::default()));
        blocks[1][2] = Some(Blocks::Harvester(Harvester::default()));
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
                    y.spawn(
                        a_usize,
                        b_usize, 
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
                            commands.entity(player_entity).with_children(|parent| {
                                y.spawn(
                                    a_usize,
                                    b_usize,
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

    pub fn is_core_placed(&self) -> bool{
        let mut result = false;
        for x in self.blocks.iter() {
            for y in x.iter() {
                if let Some(block) = y {
                    match *block {
                        Blocks::Core(_) => {result = true},
                        _ => {}
                    }
                }
            }
        }

        result
    }
}