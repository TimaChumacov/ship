use bevy::prelude::*;

use crate::game::ship_blocks::components::Blocks;

#[derive(Clone, Copy, PartialEq)]
pub enum BlockOrigin {
    Grid,
    Loot
}

#[derive(Clone, Copy)]
pub enum BlockIndex {
    Grid(usize, usize),
    Loot(usize)
}

impl BlockIndex {
    pub fn get_loot_index(self) -> Result<usize, ()> {
        match self {
            BlockIndex::Loot(index) => {Ok(index)}
            _ => {Err(())}
        }
    }

    pub fn get_block_xy(self) -> Result<(usize, usize), ()> {
        match self {
            BlockIndex::Grid(x, y) => {Ok((x, y))}
            _ => {Err(())}
        }
    }
}

#[derive(Resource, Default)]
pub struct DragAndDrop {
    pub block_origin: Option<BlockOrigin>,
    pub hovered_block: Option<Entity>,
    pub hovered_into_loot: bool,
    pub is_block_pressed: bool,
    pub is_block_dragged: bool,
    pub targeted_block: Option<Entity>
}

#[derive(Resource, Default)]
pub struct BlockDisplay {
    pub block_to_display: Option<Blocks>,
    pub block_index: Option<BlockIndex>
    //pub loot_index: Option<usize>
}
