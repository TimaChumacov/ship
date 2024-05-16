use bevy::{prelude::*, window};
use bevy::window::PrimaryWindow;
use super::components::{Crosshair, ZOOM};

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
            ZOOM,
            ZOOM,
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
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    // let cursor_position = window_query.single().cursor_position();
    // if cursor_position.is_some() {
    //     let mut crosshair_transform = crosshair_query.single_mut();
    //     crosshair_transform.translation = cursor_position.unwrap().extend(0.0)
    // }
    let mut window = window_query.get_single().unwrap();
    //window.cursor.visible = false;
    let cursor_position = window_query.single().cursor_position().unwrap_or_default();
    let mut crosshair_transform = crosshair_query.single_mut();
    crosshair_transform.translation = Vec3::new(
        (cursor_position.x - window.width() / 2.0) * ZOOM + window.width() / 2.0, 
        (cursor_position.y - window.height() / 2.0) * ZOOM + window.height() / 2.0, 
        0.0
    );
}