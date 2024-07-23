use bevy::prelude::*;
use rand::*;
use super::{ core::Core, harvester::Harvester, traits::*, turret::Turret};

#[derive(Component)]
pub struct Block {
    //corresponding x and y indexes of the ship_layout vector element this block is reading from
    pub x: usize,
    pub y: usize,
}

impl Block {
    pub fn location_by_index(
        x: usize,
        y: usize,
    ) -> Vec3 {
        let (a, b) = (x as f32, y as f32);
        
        Vec3::new(a * 32.0 - 64.0, b * -32.0 + 64.0, 0.0)
    }
}

#[derive(Clone, PartialEq)]
pub enum Blocks {
    Core(Core),
    Turret(Turret),
    Harvester(Harvester),
}


impl Spawn for Blocks {
    fn spawn(
        &self,
        x: usize,
        y: usize,
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>
        ) {
        match self {
            Self::Core(core) => core.spawn(x, y, parent, asset_server),
            Self::Turret(turret) => turret.spawn(x, y, parent, asset_server),
            Self::Harvester(harvester) => harvester.spawn(x, y, parent, asset_server),
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

    fn rotate_90(&mut self, direction: RotDirection) {
        match self {
            Self::Core(_core) => {},
            Self::Turret(turret) => turret.rotate_90(direction),
            Self::Harvester(harvester) => harvester.rotate_90(direction),
        }
    }
}

impl Description for Blocks {
    fn get_info(&self) -> String {
        match self {
            Self::Core(core) => {core.get_info()},
            Self::Turret(turret) => {turret.get_info()},
            Self::Harvester(harvester) => {harvester.get_info()}
        }
    }

    fn get_stats(&self) -> String {
        match self {
            Self::Core(core) => {core.get_stats()},
            Self::Turret(turret) => {turret.get_stats()},
            Self::Harvester(harvester) => {harvester.get_stats()}
        }
    }

    fn get_title(&self) -> String {
        match self {
            Self::Core(core) => {core.get_title()},
            Self::Turret(turret) => {turret.get_title()},
            Self::Harvester(harvester) => {harvester.get_title()}
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
