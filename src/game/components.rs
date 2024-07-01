use bevy::prelude::*;
use rand::Rng;

#[derive(Event)]
pub struct DamagedEvent(pub Entity);

#[derive(Event)]
pub struct CollisionEvent(pub Entity, pub Entity);

#[derive(Event)]
pub struct EnemyDeathEvent(pub Entity);

#[derive(Event)]
pub struct BlockDestructionEvent(pub Entity);

#[derive(Component)]
pub struct Destructible {
    pub hp: i8,
    pub max_hp: i8,
    pub time_spent_red: f32,
}

impl Destructible {
    pub fn damage(&mut self, damage: i8, damaged_entity: Entity, ev_damaged: &mut EventWriter<DamagedEvent>) {
        self.hp -= damage;
        ev_damaged.send(DamagedEvent(damaged_entity));
    }
}

#[derive(PartialEq)]
pub enum CollisionResponse {
    Moves,
    Stays,
    Trigger
}

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
    pub collision_response: CollisionResponse,
}

impl Default for Collider {
    fn default() -> Self {
        Collider {
            radius: 16.0,
            collision_response: CollisionResponse::Moves,
        }
    }
}

#[derive(Component)]
pub struct Loot {
    pub is_targeted: bool,
    pub speed_mult: f32,
    pub dir: Vec3,
}

impl Default for Loot {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let dir = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        Loot {
            is_targeted: false,
            speed_mult: 3.0,
            dir: dir
        }
    }
}