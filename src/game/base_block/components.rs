use bevy::prelude::*;

pub const PLAYER_SPEED: f32 = 100.0;

#[derive(Component)]
pub struct BaseBlock {}

#[derive(Component)]
pub struct Grappler {
    pub target: Vec3,
}