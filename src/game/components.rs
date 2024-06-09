use bevy::prelude::*;

#[derive(Event)]
pub struct DamagedEvent(pub Entity);

#[derive(Component)]
pub struct Destructible {
    pub hp: i8,
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
}