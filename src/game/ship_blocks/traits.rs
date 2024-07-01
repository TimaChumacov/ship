use bevy::prelude::*;

use crate::game::Destructible;

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

pub trait Description {
    fn get_info() -> String {
        format!("Description here...")
    }

    fn get_stats(target: &Destructible) -> String {
        format!("hp: {}/{} \n", target.hp, target.max_hp)
    }
}