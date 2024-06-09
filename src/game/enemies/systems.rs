use bevy::prelude::*;
use crate::game::ship_blocks::components::Block;

use super::components::{ChaseBehavior, EnemySpawnTimer};
use super::enemy_1::*;

pub fn spawn_enemies(
    commands: Commands,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    enemy_spawn_timer.timer.tick(time.delta());
    
    if enemy_spawn_timer.timer.finished() {
        Enemy1::spawn(commands, asset_server);
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &ChaseBehavior)>,
    blocks_query: Query<&GlobalTransform, (With<Block>, Without<ChaseBehavior>)>,
    time: Res<Time>
) {
    for (mut enemy_transform, enemy_chase) in enemy_query.iter_mut() {
        let mut closest_target: Option<Vec3> = None;
        for block_glob_transform in blocks_query.iter() {
            if closest_target.is_none() || 
               enemy_transform.translation.distance(block_glob_transform.translation()) < enemy_transform.translation.distance(closest_target.unwrap()) {
                closest_target = Some(block_glob_transform.translation());
            }
        }
        let dir = (closest_target.unwrap() - enemy_transform.translation).normalize();
        enemy_transform.translation += dir * enemy_chase.speed * time.delta_seconds();
        enemy_transform.rotation = Quat::from_rotation_arc(Vec3::Y, dir);
    }
}