use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use super::components::Crosshair;

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            window.width() / 2.0,
            window.height() / 2.0,
            0.0,
        ).with_scale(Vec3::new(
            0.5,
            0.5,
            1.0,
        )),
        ..default()
    });
}

pub fn spawn_crosshair(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/crosshair.png"),
            ..default()
        },
        Crosshair {},
    ));
}

pub fn crosshair_follow_mouse(
    mut crosshair_query: Query<&mut Transform, With<Crosshair>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let mut crosshair_transform = crosshair_query.single_mut();
    crosshair_transform.translation = window_query.single().cursor_position().unwrap().extend(0.0); 
}