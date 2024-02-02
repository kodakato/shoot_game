use bevy::prelude::*;

pub const MAX_ROTATION_SPEED: f32 = 4.0;
pub const MAX_SPEED: f32 = 100.0;
pub const ACCELERATION: f32 = 400.0;
pub const ROTATION_ACCELERATION: f32 = 20.0;
pub const ROTATION_DECELERATION: f32 = ROTATION_ACCELERATION * 0.1;
pub const PLAYER_SCALE: Vec3 = Vec3::new(0.2,0.2,0.2);


#[derive(Component)]
pub struct Player {
    pub max_speed: f32,
    pub acceleration: f32,
    pub velocity: Vec3,
    pub rotation_velocity: f32,
    pub rotation_acceleration: f32,
    pub health: i32,
}
