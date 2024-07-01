use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::general::components::SceneElement;

use super::components::*;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    ship_layout: Res<ShipLayout>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0,
                window.height() / 2.0,
                0.0,
            ),
            texture: asset_server.load("sprites/grid.png"),
            ..default()
        }, 
        SceneElement {},
        Player {}
    )).with_children(|parent|{
        ship_layout.spawn_ship(parent, &asset_server);
    });
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            player_transform.translation.y += PLAYER_SPEED * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            player_transform.translation.x -= PLAYER_SPEED * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            player_transform.translation.y -= PLAYER_SPEED * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            player_transform.translation.x += PLAYER_SPEED * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyE) {
            player_transform.rotate_z(-time.delta_seconds());
        }
        if keyboard_input.pressed(KeyCode::KeyQ) {
            player_transform.rotate_z(time.delta_seconds());
        }
    }
}