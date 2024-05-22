use bevy::prelude::*;
use crate::game::player::components::Player;

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
    player_query: Query<&Transform, (With<Player>, Without<ChaseBehavior>)>,
    time: Res<Time>
) {
    let player_transform = player_query.single();
    for (mut enemy_transform, enemy_chase) in enemy_query.iter_mut() {
        let dir_to_player = (player_transform.translation - enemy_transform.translation).normalize();
        enemy_transform.translation += dir_to_player * enemy_chase.speed * time.delta_seconds();
        enemy_transform.rotation = Quat::from_rotation_arc(Vec3::Y, dir_to_player);
    }
}