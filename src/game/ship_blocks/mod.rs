use bevy::prelude::*;

pub mod components;

pub mod harvester;
pub mod core;
pub mod turret;

pub mod systems;
use systems::*;

use self::turret::TurretTimer;

pub struct ShipBlocksPlugin;

impl Plugin for ShipBlocksPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TurretTimer>()
        .add_system(turret_logic)
        .add_system(bullet_logic)
        .add_system(harvester_logic);
    }
}