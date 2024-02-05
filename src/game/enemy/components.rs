use bevy::prelude::*;

// Enemy
pub const ENEMY_SCALE: Vec3 = Vec3::new(0.1, 0.1, 0.0);
pub const ENEMY_SPAWN_TIME: f32 = 2.0;

#[derive(Component)]
pub struct Enemy;