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

#[derive(Resource)]
pub struct DifficultyScaling {
    pub time_played: f32, // in seconds
    pub start_enemy_spawnrate: f32, // how many seconds it takes for an enemy to spawn
    pub start_loot_droprate: f32, // the chance for an enemy to drop loot
    pub max_scale_time: f32, // time mark, after which difficulty doesn't change
    pub max_enemy_spawnrate: f32, // max spawnrate at max difficulty
    pub max_loot_droprate: f32, // max spawnrate at max difficulty
}

impl Default for DifficultyScaling {
    fn default() -> Self {
        DifficultyScaling {
            time_played: 0.0,
            start_enemy_spawnrate: 3.0,
            start_loot_droprate: 50.0,
            max_scale_time: 160.0,
            max_enemy_spawnrate: 0.3,
            max_loot_droprate: 10.0,
        }
    }
}

impl DifficultyScaling {
    pub fn get_difficulty_modifier(&self) -> f32 {
        if self.time_played < self.max_scale_time {
            self.time_played / self.max_scale_time
        } else {
            1.0
        }
    }

    pub fn get_enemy_spawnrate(&self) -> f32 {
        self.start_enemy_spawnrate - (self.start_enemy_spawnrate - self.max_enemy_spawnrate) * self.get_difficulty_modifier()
    }

    pub fn get_loot_droprate(&self) -> f32 {
        self.start_loot_droprate - (self.start_loot_droprate - self.max_loot_droprate) * self.get_difficulty_modifier()
    }
}