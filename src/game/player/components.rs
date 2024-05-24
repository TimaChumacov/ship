use bevy::{prelude::*, transform::commands};
use crate::game::ship_blocks::components::Blocks;

pub const PLAYER_SPEED: f32 = 100.0;

#[derive(Component)]
pub struct Player {}

#[derive(Resource)]
pub struct ShipLayout {
    pub blocks: Vec<Vec<Option<Blocks>>>,
}

impl Default for ShipLayout {
    fn default() -> Self {
        let mut blocks: Vec<Vec<Option<Blocks>>> = vec![vec![None; 5]; 5];
        blocks[0][0] = Some(Blocks::Core);
        blocks[2][2] = Some(Blocks::Turret);
        blocks[4][4] = Some(Blocks::Harvester);
        ShipLayout {
            blocks: blocks
        }
    }
}

impl ShipLayout {
    pub fn update_ship(&self, parent: &mut ChildBuilder, asset_server: Res<AssetServer>) {
        for (a_usize, x) in self.blocks.iter().enumerate() {
            for (b_usize, y) in x.iter().enumerate() {
                if let Some(y) = y {
                    let (a, b) = (a_usize as f32, b_usize as f32);
                    y.spawn(
                        Vec3::new(a * 32.0 - 64.0, b * -32.0 + 64.0, 0.0), 
                        parent, 
                        &asset_server
                    );
                }
            }
        }
    }

    pub fn despawn_old_blocks(
        mut commands: Commands,
        player_query: Query<Entity, With<Player>>,
    ) {
        let player_entity = player_query.single();
        commands.entity(player_entity).despawn_descendants();
    }
}