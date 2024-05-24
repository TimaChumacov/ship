use bevy::prelude::*;

#[derive(Component)]
pub struct ShipEditMenu {}

#[derive(Component)]
pub struct UiBlock {
    //corresponding x and y indexes of the ship_layout vector element the ui block is reading from
    pub x: usize,
    pub y: usize,
}