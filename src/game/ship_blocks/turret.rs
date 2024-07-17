use bevy::prelude::*;
use crate::{game::{Collider, Destructible}, general::components::SceneElement, ui::ship_edit_ui::components::UISprite};

use super::{components::Block, traits::*};

#[derive(Component, Clone, PartialEq)]
pub struct Turret {
    pub rotation: f32,
}

impl Default for Turret {
    fn default() -> Self {
        Turret {
            rotation: 0.0,
        }
    }
}

impl Spawn for Turret {
    fn spawn(
        &self,
        x: usize,
        y: usize,
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
    ) {
        parent.spawn((
            SpriteBundle {
                transform: Transform::from_translation(Block::location_by_index(x, y))
                                     .with_rotation(Quat::from_rotation_z(-self.rotation.to_radians())),
                texture: asset_server.load("sprites/turret.png"),
                ..default()
            },
            SceneElement {},
            Block {
                x: x,
                y: y,
            },
            Turret {
                rotation: self.rotation,
            },
            Destructible {
                hp: 3,
                max_hp: 3,
                time_spent_red: 0.0,
            },
            Collider {
                collision_response: crate::game::CollisionResponse::Stays,
                ..default()
            }
        ));
    }

    fn spawn_ui(
        &self,
        parent: &mut ChildBuilder, 
        asset_server: &Res<AssetServer>
    ) {
        parent.spawn((
            ImageBundle {
                transform: Transform::from_rotation(Quat::from_rotation_z(self.rotation.to_radians())),
                image: asset_server.load("sprites/turret.png").into(),
                z_index: ZIndex::Global(4),
                ..default()
            },
            UISprite {}
        ));
    }
}

impl Rotate for Turret {
    fn get_rotation(&self) -> f32 {
        self.rotation
    }

    fn rotate_90_right(&mut self) {
        self.rotation += 90.0
    }
}

impl Description for Turret {
    fn get_info(&self) -> String {
        format!("Turrets shoot in the direction they're facing. You can rotate them with [R]. Each bullet deals 1 dmg, while enemies have 3 hp. Turrets have 3 max hp")
    }

    fn get_title(&self) -> String {
        format!("Turret")
    }
}

// --- Projectiles gonna have own space later ---
#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
    pub damage: i8,
    pub lifetime: f32,
    pub is_offseted: bool,
}

impl Default for Bullet {
    fn default() -> Self {
        Bullet {
            speed: 150.0,
            damage: 1,
            lifetime: 1.0,
            is_offseted: false,
        }
    }
}

#[derive(Resource)]
pub struct TurretTimer {
    pub timer: Timer,
}

impl Default for TurretTimer {
    fn default() -> Self {
        TurretTimer { timer: Timer::from_seconds(0.3, TimerMode::Repeating) }
    }
}