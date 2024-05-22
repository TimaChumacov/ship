use bevy::prelude::*;

#[derive(Component)]
pub struct Destructible {
    pub hp: i8,
}

#[derive(Component)]
pub struct Loot {}