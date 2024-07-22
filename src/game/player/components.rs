use bevy::prelude::*;
use crate::game::ship_blocks::core::Core;
use crate::game::ship_blocks::harvester::Harvester;
use crate::game::ship_blocks::turret::Turret;
use crate::game::ship_blocks::{components::Blocks, traits::Spawn};

pub const PLAYER_SPEED: f32 = 100.0;

#[derive(Component)]
pub struct Player {}

impl Player {
    pub fn get_rotation(transform: &Transform) -> f32 {
        transform.rotation.to_euler(EulerRot::ZXY).0.to_degrees()
    }
}

#[derive(Resource)]
pub struct ShipLayout {
    pub blocks: Vec<Vec<Option<Blocks>>>,
    pub old_blocks: Vec<Vec<Option<Blocks>>>,
    //pub dragged_block: Option<Entity>, // when a block is moved after it was already placed (so it's not taken outta loot) it's stored here
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
            //dragged_block: None
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