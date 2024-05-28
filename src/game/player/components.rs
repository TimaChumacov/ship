use bevy::{ecs::entity, prelude::*, transform::commands};
use crate::game::ship_blocks::components::Blocks;

pub const PLAYER_SPEED: f32 = 100.0;

#[derive(Component)]
pub struct Player {
    pub looted_blocks: Vec<Blocks>,
    pub selected_loot: Option<Blocks>,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            looted_blocks: vec![
                Blocks::Core,
                Blocks::Turret,
                Blocks::Harvester
            ],
            selected_loot: None,
        }
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
        blocks[1][0] = Some(Blocks::Core);
        blocks[1][2] = Some(Blocks::Turret);
        blocks[3][4] = Some(Blocks::Harvester);
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