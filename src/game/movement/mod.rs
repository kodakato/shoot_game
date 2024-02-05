use bevy::prelude::*;
pub struct MovementPlugin;

mod systems;
pub mod components;

use systems::*;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_position, 
            update_velocity, 
            update_rotation,
            update_rotation_velocity,
        ));
    }
}

pub const MAX_ANGULAR_VELOCITY: f32 = 8.0;
pub const MAX_VELOCITY: f32 = 2000.0;
pub const ACCELERATION: f32 = 400.0;
pub const ANGULAR_ACCELERATION: f32 = 20.0;
pub const ANGULAR_DECELERATION_SCALE: f32 = 0.1;

