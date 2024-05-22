use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use super::components::*;
use crate::game::ship_blocks::{components::Blocks, core::Core, harvester::Harvester, turret::Turret};

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
        Player {},
    )).with_children(|parent|{
        for (a_usize, x) in ship_layout.blocks.iter().enumerate() {
            for (b_usize, y) in x.iter().enumerate() {
                if let Some(y) = y {
                    let (a, b) = (a_usize as f32, b_usize as f32);
                    y.spawn(
                        Vec3::new(a * 32.0 - 64.0, b * -32.0 + 64.0, 0.0), 
                        parent, 
                        &asset_server
                    );
                }
            }
        }
    });
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
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