use bevy::prelude::*;

pub const ZOOM: f32 = 0.5;

#[derive(Component)] 
pub struct Camera {}

#[derive(Component)]
pub struct Background {
    pub anchor_point: Vec3
}

#[derive(Component)]
pub struct SceneElement {}