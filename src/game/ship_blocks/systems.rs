use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use super::components::Block;
use super::core::Core;
use super::turret::{Turret, Bullet, TurretTimer};
use super::harvester::{Wire, Grappler, Harvester};
use crate::game::ship_blocks::components::Blocks;
use crate::game::player::components::{Player, ShipLayout};
use crate::game::player::player_loot::PlayerLoot;
use crate::game::{components::*, enemies::components::Enemy};
use crate::general::components::SceneElement;
use crate::general::states::AppState;

pub fn block_destruction(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    mut ship_layout: ResMut<ShipLayout>,
    block_query: Query<(&Block, Entity, Option<&Core>)>,
    mut block_destruction_ev: EventReader<BlockDestructionEvent>,
) {
    for ev in block_destruction_ev.read() {
        if let Ok((block, block_entity, core)) = block_query.get(ev.0) {
            ship_layout.blocks[block.x][block.y] = None;
            commands.entity(block_entity).despawn_recursive();
            if core.is_some() {
                next_state.set(AppState::MainMenu);
            }
        }
    }
}

pub fn turret_logic(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    player_query: Query<&Transform, With<Player>>,
    turret_query: Query<(&GlobalTransform, &Turret)>,
    mut turret_timer: ResMut<TurretTimer>,
    time: Res<Time>
) {
    turret_timer.timer.tick(time.delta());
    if turret_timer.timer.finished() {
        if let Ok(player_transform) = player_query.get_single() {
            if !turret_query.is_empty() {
                audio.play(asset_server.load("audio/bullet_shot.ogg"));
            }
            for (turret_glob_transform, turret) in turret_query.iter() {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_translation(
                            turret_glob_transform.translation()
                        ).with_rotation(Quat::from_rotation_z((-turret.rotation + Player::get_rotation(player_transform)).to_radians())),
                        visibility: Visibility::Hidden,
                        texture: asset_server.load("sprites/bullet.png"),
                        ..default()
                    },
                    SceneElement {},
                    Bullet::default(),
                ));
            }
        }
    }
}

pub fn bullet_logic(
    mut commands: Commands,
    mut bullet_query: Query<(&mut Transform, &mut Visibility, &mut Bullet, Entity)>,
    mut enemy_query: Query<(&Transform, Entity, &mut Destructible), (With<Enemy>, Without<Bullet>)>,
    mut ev_damaged: EventWriter<DamagedEvent>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>
) {
    for (mut bullet_transform, mut bullet_visibility, mut bullet_stats, bullet_entity) in bullet_query.iter_mut() {
        if !bullet_stats.is_offseted {
            let offset = bullet_transform.local_y() * 16.0;
            bullet_transform.translation += offset;
            *bullet_visibility = Visibility::Visible;
            bullet_stats.is_offseted = true;
        }
        let movement = bullet_transform.local_y() * bullet_stats.speed * time.delta_seconds();
        bullet_transform.translation += movement;
        bullet_stats.lifetime -= time.delta_seconds();
        if bullet_stats.lifetime <= 0.0 {
            commands.entity(bullet_entity).despawn();
        }
        for (enemy_transform, enemy_entity, mut enemy_destructible) in enemy_query.iter_mut() {
            if bullet_transform.translation.distance(enemy_transform.translation) < 17.0 {
                audio.play(asset_server.load("audio/enemy_damaged.ogg"));
                enemy_destructible.damage(1, enemy_entity, &mut ev_damaged);
                commands.entity(bullet_entity).despawn();
            }
        }
    }
}

pub fn harvester_logic(
    mut commands: Commands,
    mut harvester_query: Query<(&GlobalTransform, Entity, &mut Harvester)>,
    mut loot_query: Query<(&Transform, Entity, &mut Loot)>, 
    asset_server: Res<AssetServer>,
    audio: Res<Audio>
) {
    for (loot_transform, loot_entity, mut loot) in loot_query.iter_mut() {
        for (harvester_glob_transform, harvester_entity, mut harvester) in harvester_query.iter_mut() {
            let distance_to_loot = loot_transform.translation.distance(harvester_glob_transform.translation());
            // new grappler will be assigned to loot only if: it doesn't already transport loot; the loot isn't targeted by other Grappler; loot is within set distance;
            if !harvester.deployed_grappler && !loot.is_targeted && distance_to_loot < 80.0 {
                audio.play(asset_server.load("audio/rope.ogg"));
                // spawns a new grappler and sets its target
                commands.entity(harvester_entity).despawn_descendants();
                harvester.spawn_global_grappler(
                    harvester_glob_transform.translation(), 
                    Some(loot_entity), 
                    Some(harvester_entity), 
                    &mut commands, 
                    &asset_server
                );
                loot.is_targeted = true;
                harvester.deployed_grappler = true;
            }
        }
    }
}

pub fn grappler_logic(
    mut commands: Commands,
    time: Res<Time>,
    mut player_loot: ResMut<PlayerLoot>,
    mut loot_query: Query<(&mut Transform, Entity, &mut Loot), Without<Grappler>>,
    mut base_query: Query<(&GlobalTransform, Entity, &mut Harvester)>,
    mut grappler_query: Query<(&mut Transform, Entity, &mut Grappler, &Children)>,
    mut wire_query: Query<&mut Transform, (With<Wire>, Without<Grappler>, Without<Loot>)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for (mut grappler_transform, grappler_entity, mut grappler, children) in grappler_query.iter_mut() {
        if let Ok((mut loot_transform, loot_entity, mut loot)) = loot_query.get_mut(grappler.target.unwrap()) {
            let mut wire_transform = wire_query.get_mut(*children.first().unwrap()).unwrap();
            if let Ok((base_glob_transform, base_entity, mut base)) = base_query.get_mut(grappler.base.unwrap()) {
                // defining rotation since grappler should always face away from base
                let dir_away_from_base = (grappler_transform.translation - base_glob_transform.translation()).normalize();
                grappler_transform.rotation = Quat::from_rotation_arc(Vec3::Y, dir_away_from_base);
                // logic for grappler flying to the loot
                if !grappler.is_returning {
                    let dir = (loot_transform.translation - grappler_transform.translation).normalize();
                    grappler_transform.translation += dir * time.delta_seconds() * 180.0;
                    // if loot is close it's grabbed, if too far the grappler returns empty
                    if grappler_transform.translation.distance(loot_transform.translation) < 20.0 {
                        audio.play(asset_server.load("audio/grappler-pick-up.ogg"));
                        grappler.grabbed_loot = true;
                        grappler.is_returning = true;
                    } else if base_glob_transform.translation().distance(loot_transform.translation) > 80.0 {
                        loot.is_targeted = false;
                        grappler.is_returning = true;
                    }
                // logic for grappler flying back
                } else {
                    let dir = (base_glob_transform.translation() - grappler_transform.translation).normalize();
                    grappler_transform.translation += dir * time.delta_seconds() * 100.0;
                    // logic if flying back with loot
                    if grappler.grabbed_loot {
                        loot_transform.translation = grappler_transform.translation;
                        loot_transform.rotation = grappler_transform.rotation;
                    }
                    // resetting everything once grappler returns
                    if base_glob_transform.translation().distance(grappler_transform.translation) < 5.0 {
                        if grappler.grabbed_loot {
                            commands.entity(loot_entity).despawn();
                        }
                        player_loot.put_block_in_loot(&Blocks::get_random_block());
                        base.deployed_grappler = false;
                        commands.entity(base_entity).with_children(|parent| {
                            base.spawn_child_grappler(parent, &asset_server);
                        });
                        commands.entity(grappler_entity).despawn_recursive();
                    }
                }
                // wire logic that is always relevant
                let wire_length = (grappler_transform.translation - base_glob_transform.translation()).xy().length();
                wire_transform.translation.y = -wire_length / 2.0;
                wire_transform.scale.y = wire_length / 32.0;
            } else {
                // happens if the base harvester is despawned (e.g. player removed it in ship edit while loot was being collected)
                loot.is_targeted = false;
                commands.entity(grappler_entity).despawn_recursive();
            }
        }
    }
}