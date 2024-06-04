use bevy::prelude::*;
use distributions::{Distribution, Standard};
use rand::*;
use super::{ traits::*, core::Core, turret::Turret, harvester::Harvester};

#[derive(Component)]
pub struct Block {}

#[derive(Clone, PartialEq)]
pub enum Blocks {
    Core(Core),
    Turret(Turret),
    Harvester(Harvester),
}


impl Spawn for Blocks {
    fn spawn(
        &self,
        spawn_pos: Vec3, 
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>
        ) {
        match self {
            Self::Core(core) => core.spawn(spawn_pos, parent, asset_server),
            Self::Turret(turret) => turret.spawn(spawn_pos, parent, asset_server),
            Self::Harvester(harvester) => harvester.spawn(spawn_pos, parent, asset_server),
        }
    }

    fn spawn_ui(
        &self,
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>
        ) {
        match self {
            Self::Core(core) => core.spawn_ui(parent, asset_server),
            Self::Turret(turret) => turret.spawn_ui(parent, asset_server),
            Self::Harvester(harvester) => harvester.spawn_ui(parent, asset_server),
        }
    }
}

impl Rotate for Blocks {
    fn get_rotation(&self) -> f32 {
        match self {
            Self::Core(_core) => 0.0,
            Self::Turret(turret) => turret.get_rotation(),
            Self::Harvester(harvester) => harvester.get_rotation(),
        }
    }

    fn rotate_90_right(&mut self) {
        match self {
            Self::Core(_core) => {},
            Self::Turret(turret) => turret.rotate_90_right(),
            Self::Harvester(harvester) => harvester.rotate_90_right(),
        }
    }
}

impl Blocks {
    pub fn get_random_block() -> Blocks {
        match thread_rng().gen_range(0..=1) {
            0 => Self::Turret(Turret::default()),
            1 => Self::Harvester(Harvester::default()),
            _ => Self::Core(Core::default()),
        }
    }
}
