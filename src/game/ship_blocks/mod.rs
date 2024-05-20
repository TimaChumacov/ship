use bevy::prelude::*;

pub mod components;

pub mod harvester;
pub mod core;
pub mod turret;

pub mod systems;
use systems::*;

pub struct BaseBlockPlugin;

impl Plugin for BaseBlockPlugin {
    fn build(&self, app: &mut App) {
    }
}