use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::game::player::{self, components::Player};

use super::components::{Camera, ZOOM};

pub fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(
        SpriteBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0,
                window.height() / 2.0,
                0.0,
            ).with_scale(Vec3::new(1.0, 1.0, 0.0)),
            texture: asset_server.load("sprites/background.png"),
            ..default()
        }
    );
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0,
                window.height() / 2.0,
                0.0,
            ).with_scale(Vec3::new(
                ZOOM,
                ZOOM,
                1.0,
            )),
            ..default()
        },
        Camera {}
    ));
}

pub fn follow_player(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Changed<Transform>)>,
) {
    let mut camera_transform = camera_query.get_single_mut().unwrap();
    if let Ok(player_transform) = player_query.get_single() {
        //let dir = player_transform.translation - camera_transform.translation;
        camera_transform.translation = player_transform.translation;
    }
}

// pub fn spawn_crosshair(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
// ) {
//     commands.spawn((
//         SpriteBundle {
//             texture: asset_server.load("sprites/crosshair.png"),
//             ..default()
//         },
//         Crosshair {},
//     ));
// }

// pub fn crosshair_follow_mouse(
//     mut crosshair_query: Query<&mut Transform, With<Crosshair>>,
//     mut window_query: Query<&mut Window, With<PrimaryWindow>>,
// ) {
//     let mut window = window_query.get_single_mut().unwrap();
//     window.cursor.visible = false;
//     let cursor_position = window.cursor_position();
//     match cursor_position {
//         Some(cursor_position) => {
//             let mut crosshair_transform = crosshair_query.single_mut();
//             crosshair_transform.translation = Vec3::new(
//                 (cursor_position.x - window.width() / 2.0) * ZOOM + window.width() / 2.0, 
//                 (cursor_position.y - window.height() / 2.0) * ZOOM + window.height() / 2.0, 
//                 0.0
//             );
//         },
//         None => {}
//     }
// }