use bevy::prelude::*;

// Enemy
const ENEMY_SIZE: Vec2 = Vec2::new(20.0, 20.0);
const ENEMY_SPAWN_SECONDS: u64 = 2;

#[derive(Component)]
struct Enemy;