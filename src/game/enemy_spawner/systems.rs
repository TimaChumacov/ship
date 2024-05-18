use bevy::prelude::*;
use crate::game::base_block::components::BaseBlock;

use super::components::{ChaseBehavior, EnemySpawnTimer};
use super::enemy_1::*;

pub fn tick_enemy_spawn_timer(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>
) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies(
    commands: Commands,
    enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    asset_server: Res<AssetServer>,
) {
    if enemy_spawn_timer.timer.finished() {
        Enemy1::spawn(commands, asset_server);
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &ChaseBehavior)>,
    player_query: Query<&Transform, (With<BaseBlock>, Without<ChaseBehavior>)>,
    time: Res<Time>
) {
    let player_transform = player_query.single();
    for (mut enemy_transform, enemy_chase) in enemy_query.iter_mut() {
        let dir_to_player = (player_transform.translation - enemy_transform.translation).normalize();
        enemy_transform.translation += dir_to_player * enemy_chase.speed * time.delta_seconds();
        enemy_transform.rotation = Quat::from_rotation_arc(Vec3::Y, dir_to_player);
    }
}