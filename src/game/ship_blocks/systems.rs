use bevy::prelude::*;
use super::turret::{Turret, Bullet, TurretTimer};
use super::harvester::{self, Harvester};
use crate::game::{components::{Destructible, Loot}, enemies::components::Enemy};

pub fn turret_logic(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    turret_query: Query<&GlobalTransform, With<Turret>>,
    mut turret_timer: ResMut<TurretTimer>,
    time: Res<Time>
) {
    turret_timer.timer.tick(time.delta());
    if turret_timer.timer.finished() {
        for turret_glob_transform in turret_query.iter() {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        turret_glob_transform.translation().x,
                        turret_glob_transform.translation().y + 16.0,
                        0.0
                    ),
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
        bullet_transform.translation.y += bullet_stats.speed * time.delta_seconds();
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
    harvester_query: Query<&GlobalTransform, With<Harvester>>,
    loot_query: Query<(&Transform, Entity), With<Loot>>
) {
    for (loot_transform, loot_entity) in loot_query.iter() {
        for harvester_glob_transform in harvester_query.iter(){
            if loot_transform.translation.distance(harvester_glob_transform.translation()) < 80.0 {
                commands.entity(loot_entity).despawn();
            }
        }
    }
}

// pub fn spawn_player(
//     mut commands: Commands,
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     asset_server: Res<AssetServer>,
// ) {
//     let window = window_query.get_single().unwrap();

//     commands.spawn((
//         SpriteBundle {
//             transform: Transform::from_xyz(
//                 window.width() / 2.0,
//                 window.height() / 2.0,
//                 0.0,
//             ),
//             texture: asset_server.load("sprites/base.png"),
//             ..default()
//         },
//         BaseBlock {},
//     )).with_children(|parent| {
//         parent.spawn((
//             SpriteBundle {
//                 transform: Transform::from_xyz(
//                     0.0,
//                     0.0,
//                     0.0,
//                 ),
//                 texture: asset_server.load("sprites/grappler.png"),
//                 ..default()
//             },
//             Grappler {},
//         ));
//     });
// }

// pub fn player_movement(
//     keyboard_input: Res<Input<KeyCode>>,
//     mut player_query: Query<&mut Transform, With<BaseBlock>>,
//     time: Res<Time>,
// ) {
//     if let Ok(mut player_transform) = player_query.get_single_mut() {
//         if keyboard_input.pressed(KeyCode::W) {
//             player_transform.translation.y += PLAYER_SPEED * time.delta_seconds();
//         }
//         if keyboard_input.pressed(KeyCode::A) {
//             player_transform.translation.x -= PLAYER_SPEED * time.delta_seconds();
//         }
//         if keyboard_input.pressed(KeyCode::S) {
//             player_transform.translation.y -= PLAYER_SPEED * time.delta_seconds();
//         }
//         if keyboard_input.pressed(KeyCode::D) {
//             player_transform.translation.x += PLAYER_SPEED * time.delta_seconds();
//         }
//     }
// }

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
