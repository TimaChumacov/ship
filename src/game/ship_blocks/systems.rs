use bevy::prelude::*;
use super::turret::{Turret, Bullet, TurretTimer};
use super::harvester::{Wire, Grappler, Harvester};
use crate::game::ship_blocks::components::Blocks;
use crate::game::player::components::PlayerLoot;
use crate::game::{components::{Destructible, Loot}, enemies::components::Enemy};

pub fn turret_logic(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    turret_query: Query<(&GlobalTransform, &Transform, &Turret)>,
    mut turret_timer: ResMut<TurretTimer>,
    time: Res<Time>
) {
    turret_timer.timer.tick(time.delta());
    if turret_timer.timer.finished() {
        for (turret_glob_transform, turret_transform, turret) in turret_query.iter() {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(
                        turret_glob_transform.translation() + turret_transform.local_y() * 16.0
                    ).with_rotation(turret_transform.rotation),
                    texture: asset_server.load("sprites/bullet.png"),
                    ..default()
                },
                Bullet {
                    speed: 150.0,
                    damage: 1,
                    lifetime: 1.0,
                }
            ));
        }
    }
}

pub fn bullet_logic(
    mut commands: Commands,
    mut bullet_query: Query<(&mut Transform, &mut Bullet, Entity)>,
    mut enemy_query: Query<(&Transform, &mut Destructible), (With<Enemy>, Without<Bullet>)>,
    time: Res<Time>
) {
    for (mut bullet_transform, mut bullet_stats, bullet_entity) in bullet_query.iter_mut() {
        let movement = bullet_transform.local_y() * bullet_stats.speed * time.delta_seconds();
        bullet_transform.translation += movement;
        bullet_stats.lifetime -= time.delta_seconds();
        if bullet_stats.lifetime <= 0.0 {
            commands.entity(bullet_entity).despawn();
        }
        for (enemy_transform, mut enemy_destructible) in enemy_query.iter_mut() {
            if bullet_transform.translation.distance(enemy_transform.translation) < 17.0 {
                enemy_destructible.hp -= bullet_stats.damage;
            }
        }
    }
}

pub fn harvester_logic(
    mut commands: Commands,
    time: Res<Time>,
    mut player_loot: ResMut<PlayerLoot>,
    harvester_query: Query<(&Harvester, &Children)>,
    mut grappler_query: Query<(&GlobalTransform, &mut Transform, &mut Grappler, &Children)>,
    mut wire_query: Query<&mut Transform, (With<Wire>, Without<Grappler>, Without<Loot>)>,
    mut loot_query: Query<(&mut Transform, Entity, &mut Loot), Without<Grappler>>
) {
    for (mut loot_transform, loot_entity, mut loot) in loot_query.iter_mut() {
        for (harvester, children) in harvester_query.iter() {
            let child = children.first().unwrap();
            let (grappler_glob_transform, mut grappler_transform, mut grappler, grappler_children) = grappler_query.get_mut(*child).unwrap();
            let wire_entity = grappler_children.first().unwrap();
            let mut wire_transform = wire_query.get_mut(*wire_entity).unwrap();
            let distance_to_loot = loot_transform.translation.distance(grappler_glob_transform.translation());
            if grappler.target.is_none() && !loot.is_targeted && distance_to_loot < 80.0 {
                grappler.target = Some(loot_entity);
                loot.is_targeted = true;
            } else if grappler.target.is_some() && grappler.target.unwrap() == loot_entity {
                if !grappler.is_returning {
                    let dir = (loot_transform.translation - grappler_glob_transform.translation()).normalize();
                    grappler_transform.translation += dir * time.delta_seconds() * 80.0;
                    grappler_transform.rotation = Quat::from_rotation_arc(Vec3::Y, dir);
                    if distance_to_loot < 20.0 {
                        grappler.grabbed_loot = true;
                        grappler.is_returning = true;
                    } else if grappler_transform.translation.length() > 80.0 {
                        loot.is_targeted = false;
                        grappler.is_returning = true;
                    }
                } else {
                    let dir = -grappler_transform.translation.normalize();
                    grappler_transform.translation += dir * time.delta_seconds() * 40.0;
                    grappler_transform.rotation = Quat::from_rotation_arc(Vec3::Y, -dir);
                    if grappler.grabbed_loot {
                        loot_transform.translation = grappler_glob_transform.translation();
                        loot_transform.rotation = grappler_transform.rotation;
                    }
                    if grappler_transform.translation.length() < 5.0 {
                        if grappler.grabbed_loot {
                            commands.entity(loot_entity).despawn();
                        }
                        player_loot.put_block_in_loot(&Blocks::get_random_block());
                        grappler.target = None;
                        grappler.is_returning = false;
                        grappler.grabbed_loot = false;
                        grappler_transform.translation = Vec2::ZERO.extend(1.0);
                        grappler_transform.rotation = Quat::from_rotation_z(harvester.rotation.to_radians());
                    }
                }
                let wire_length = grappler_transform.translation.xy().length();
                wire_transform.translation.y = -wire_length / 2.0;
                wire_transform.scale.y = wire_length / 32.0;
            }
        }
    }
}

// pub fn grappler_logic(
//     mut grappler_query: Query<(&mut Transform, &GlobalTransform), With<Grappler>>,
//     crosshair_query: Query<&Transform, (With<Crosshair>, Without<Grappler>)>
// ) {
//     let (mut grappler_transform, grappler_glob_transform) = grappler_query.single_mut();
//     let crosshair_transform = crosshair_query.single();
//     let dir_to_crosshair = (crosshair_transform.translation - grappler_glob_transform.translation()).normalize();
//     grappler_transform.rotation = Quat::from_rotation_arc(Vec3::Y, dir_to_crosshair);
// }

// pub fn target_for_grappler(
//     crosshair_query: Query<&Transform, With<Crosshair>>,
//     enemy_query: Query<&Transform, With<Enemy>>,
//     mouse: Res<Input<MouseButton>>,
// ) {
//     let crosshair_transform = crosshair_query.single();
//     if mouse.just_pressed(MouseButton::Left) {
//         println!("clicked...");
//         for enemy_transform in enemy_query.iter() {
//             println!("enemy check... {}", crosshair_transform.translation.distance(enemy_transform.translation));
//             if crosshair_transform.translation.distance(enemy_transform.translation) < 32.0 {
//                 println!("clicked on an enemy!");
//             }
//         }
//     }
// }
