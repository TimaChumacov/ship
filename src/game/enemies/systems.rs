use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;
use crate::game::player::components::Player;
use crate::game::ship_blocks::components::Block;
use crate::game::{CollisionEvent, DamagedEvent, Destructible, EnemyDeathEvent, Loot};
use crate::game::components::DifficultyScaling;
use crate::general::components::ZOOM;

use super::components::{ChaseBehavior, Enemy, EnemySpawnTimer, Melee};
use super::enemy_1::*;

pub fn spawn_enemies(
    commands: Commands,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    difficulty: Res<DifficultyScaling>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&Transform, With<Player>>
) {
    enemy_spawn_timer.timer.tick(time.delta());

    let window = window_query.get_single().unwrap();
    if let Ok(player_transform) = player_query.get_single() {
        let mut rng = rand::thread_rng();
        let dir_away = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0).normalize();
        let spawn_pos = player_transform.translation + dir_away * window.width() / 2.0 * ZOOM * 1.25;
        if enemy_spawn_timer.timer.finished() {
            Enemy1::spawn(spawn_pos, commands, asset_server);
            enemy_spawn_timer.timer.set_duration(Duration::from_secs_f32(difficulty.get_enemy_spawnrate()))
        }
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
        if let Some(closest_target) = closest_target {
            let dir = (closest_target - enemy_transform.translation).normalize();
            enemy_transform.translation += dir * enemy_chase.speed * time.delta_seconds();
            enemy_transform.rotation = Quat::from_rotation_arc(Vec3::Y, dir);
        }
    }
}

pub fn enemy_collides_block(
    mut enemy_query: Query<&mut Melee>,
    mut block_query: Query<&mut Destructible, With<Block>>,
    mut collision_ev: EventReader<CollisionEvent>,
    mut damaged_ev: EventWriter<DamagedEvent>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>
) {
    for ev in collision_ev.read() {
        if let Ok(mut enemy_melee) = enemy_query.get_mut(ev.0) {
            if let Ok(mut block_destr) = block_query.get_mut(ev.1) {
                if enemy_melee.cooldown_left <= 0.0 {
                    block_destr.damage(enemy_melee.damage, ev.1, &mut damaged_ev);
                    enemy_melee.cooldown_left = enemy_melee.attack_cooldown;
                } else {
                    enemy_melee.cooldown_left -= time.delta_seconds();
                }
            }
        } else 
        if let Ok(mut enemy_melee) = enemy_query.get_mut(ev.1) {
            if let Ok(mut block_destr) = block_query.get_mut(ev.0) {
                if enemy_melee.cooldown_left <= 0.0 {
                    audio.play(asset_server.load("audio/block_damaged.ogg"));
                    block_destr.damage(enemy_melee.damage, ev.0, &mut damaged_ev);
                    enemy_melee.cooldown_left = enemy_melee.attack_cooldown;
                } else {
                    enemy_melee.cooldown_left -= time.delta_seconds();
                }
            }
        }
    }
}

pub fn enemy_death(
    mut commands: Commands,
    difficulty: Res<DifficultyScaling>,
    enemy_query: Query<(&Transform, Entity), With<Enemy>>,
    mut enemy_death_ev: EventReader<EnemyDeathEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>
) {
    for ev in enemy_death_ev.read() {
        if let Ok((enemy_transform, enemy_entity)) = enemy_query.get(ev.0) {
            audio.play(asset_server.load("audio/enemy_death.ogg")).with_volume(0.5);
            let loot_drop: f32 = rand::thread_rng().gen();
            if loot_drop * 100.0 < difficulty.get_loot_droprate() {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(enemy_transform.translation),
                    texture: asset_server.load("sprites/loot.png"),
                    ..default()
                },
                Loot::default()
            ));
            }
            commands.entity(enemy_entity).despawn_recursive();
        }
    }
}