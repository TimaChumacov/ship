use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::general::components::Crosshair;
use crate::game::enemy_spawner::components::Enemy;
use super::components::*;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0,
                window.height() / 2.0,
                0.0,
            ),
            texture: asset_server.load("sprites/base.png"),
            ..default()
        },
        BaseBlock {},
    )).with_children(|parent| {
        parent.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    0.0,
                    0.0,
                    0.0,
                ),
                texture: asset_server.load("sprites/grappler.png"),
                ..default()
            },
            Grappler {},
        ));
    });
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<BaseBlock>>,
    time: Res<Time>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            player_transform.translation.y += PLAYER_SPEED * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::A) {
            player_transform.translation.x -= PLAYER_SPEED * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::S) {
            player_transform.translation.y -= PLAYER_SPEED * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::D) {
            player_transform.translation.x += PLAYER_SPEED * time.delta_seconds();
        }
    }
}

pub fn grappler_logic(
    mut grappler_query: Query<(&mut Transform, &GlobalTransform), With<Grappler>>,
    crosshair_query: Query<&Transform, (With<Crosshair>, Without<Grappler>)>
) {
    let (mut grappler_transform, grappler_glob_transform) = grappler_query.single_mut();
    let crosshair_transform = crosshair_query.single();
    let dir_to_crosshair = (crosshair_transform.translation - grappler_glob_transform.translation()).normalize();
    grappler_transform.rotation = Quat::from_rotation_arc(Vec3::Y, dir_to_crosshair);
}

pub fn target_for_grappler(
    crosshair_query: Query<&Transform, With<Crosshair>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mouse: Res<Input<MouseButton>>,
) {
    let crosshair_transform = crosshair_query.single();
    if mouse.just_pressed(MouseButton::Left) {
        println!("clicked...");
        for enemy_transform in enemy_query.iter() {
            println!("enemy check... {}", crosshair_transform.translation.distance(enemy_transform.translation));
            if crosshair_transform.translation.distance(enemy_transform.translation) < 32.0 {
                println!("clicked on an enemy!");
            }
        }
    }
}
