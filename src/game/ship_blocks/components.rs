use bevy::prelude::*;
use super::{core::Core, turret::Turret, harvester::Harvester};

#[derive(Component)]
pub struct Block {}

pub trait Spawn {
    fn spawn(
        spawn_pos: Vec3, 
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>
    );

    fn spawn_ui(
       parent: &mut ChildBuilder, 
       asset_server: &Res<AssetServer>
    );
}
#[derive(Clone)]
pub enum Blocks {
    Core,
    Turret,
    Harvester,
}

impl Blocks {
    pub fn spawn(
        &self,
        spawn_pos: Vec3, 
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>
        ) {
        match self {
            Self::Core => Core::spawn(spawn_pos, parent, asset_server),
            Self::Turret => Turret::spawn(spawn_pos, parent, asset_server),
            Self::Harvester => Harvester::spawn(spawn_pos, parent, asset_server),
        }
    }

    pub fn spawn_ui(
        &self,
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>
        ) {
        match self {
            Self::Core => Core::spawn_ui(parent, asset_server),
            Self::Turret => Turret::spawn_ui(parent, asset_server),
            Self::Harvester => Harvester::spawn_ui(parent, asset_server),
        }
    }
}
