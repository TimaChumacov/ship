use bevy::prelude::*;

#[derive(Component)]
pub struct ShipEditMenu {}

#[derive(Component)]
pub struct UiBlock {
    //corresponding x and y indexes of the ship_layout vector element this ui block is reading from
    pub x: usize,
    pub y: usize,
}

#[derive(Component)]
pub struct SmallUiBlock {
    // index of the loot from Player.looted_blocks that this ui block represents
    pub index: usize,
}