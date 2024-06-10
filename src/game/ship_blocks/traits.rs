use bevy::prelude::*;

pub trait Spawn {
    fn spawn(
        &self,
        x: usize,
        y: usize,
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>
    );

    fn spawn_ui(
       &self,
       parent: &mut ChildBuilder, 
       asset_server: &Res<AssetServer>
    );
}

pub trait Rotate {
    fn get_rotation(&self) -> f32; 

    fn rotate_90_right(&mut self);
}