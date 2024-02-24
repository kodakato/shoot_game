use bevy::prelude::*;

use super::components::*;
use super::*;

pub fn update_position(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}

pub fn update_velocity(
    mut query: Query<(&mut Velocity, &Acceleration)>,
    time: Res<Time>,
) {
    for (mut velocity, acceleration) in query.iter_mut() {
        let new_velocity = velocity.value + acceleration.value * time.delta_seconds();
        // Check if the velocity's magnitude exceeds the maximum speed
        if new_velocity.length() > velocity.max {
            velocity.value = new_velocity.normalize() * velocity.max;
        } else {
            velocity.value = new_velocity;
        }  
    }

}

pub fn update_rotation(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &AngularVelocity)>,
) {
    for (mut transform, angular_velocity) in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_z(angular_velocity.value * time.delta_seconds());
    }
}

pub fn update_rotation_velocity (
    time: Res<Time>,
    mut query: Query<(&mut AngularVelocity, &AngularAcceleration)>,
) {
    for (mut angular_velocity, angular_acceleration) in query.iter_mut() {
        // Accelerate rotation if there is an acceleration to the max acceleration speed, otherwise decelerate
        if angular_acceleration.value != 0.0 {
            angular_velocity.value += angular_acceleration.value * time.delta_seconds();
        } else {
            let deceleration = angular_velocity.value.signum() * angular_velocity.max * ANGULAR_DECELERATION_SCALE * time.delta_seconds();
            if deceleration.abs() < angular_velocity.value.abs() {
                angular_velocity.value -= deceleration;
            } else {
                angular_velocity.value = 0.0;
            }
        }
    }
}



