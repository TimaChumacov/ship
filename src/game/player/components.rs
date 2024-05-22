use bevy::prelude::*;
use crate::game::ship_blocks::components::Blocks;

pub const PLAYER_SPEED: f32 = 100.0;

#[derive(Component)]
pub struct Player {}

#[derive(Resource)]
pub struct ShipLayout {
    pub blocks: Vec<Vec<Option<Blocks>>>,
}

impl Default for ShipLayout {
    fn default() -> Self {
        let mut blocks: Vec<Vec<Option<Blocks>>> = vec![vec![None; 5]; 5];
        blocks[2][2] = Some(Blocks::Core);
        ShipLayout {
            blocks: blocks
        }
    }
}