use bevy::prelude::*;
use crate::general::states::PauseState;

pub mod components;
pub mod traits;

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
        .add_systems(Update, (
            turret_logic, 
            bullet_logic, 
            harvester_logic,
            grappler_logic,
            block_destruction
        ).run_if(in_state(PauseState::Running)));
    }
}