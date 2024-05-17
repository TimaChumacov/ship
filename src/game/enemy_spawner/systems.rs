use bevy::prelude::*;
use super::timer::EnemySpawnTimer;
use super::enemy_1::*;

pub fn tick_enemy_spawn_timer(
    enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>
) {
    enemy_spawn_timer.timer.tick(time.delta())
}

pub fn spawn_enemies(
    mut commands: Commands,
    enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    asset_server: Res<AssetServer>,
) {
    if enemy_spawn_timer.is_finished() {
        Enemy1::spawn(commands, asset_server);
    }
}