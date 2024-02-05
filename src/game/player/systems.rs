use bevy::prelude::*;
use crate::game::movement::*;

use crate::game::movement::components::{Acceleration, AngularAcceleration, AngularVelocity, Velocity};
use crate::game::movement::components::MovingObjectBundle;

use super::components::Player;
use super::*;

// This function only sets the values for the player's movement. The actual movement is handled in the movement system.
pub fn player_movement(
    mut player_query: Query<(&mut Acceleration, &mut Transform, &mut AngularAcceleration, &mut AngularVelocity), With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    let (mut acceleration, transform, mut angular_acceleration, mut angular_velocity) = player_query.single_mut();

    // Accelerate forward
    if input.pressed(ACCELERATE_KEY) {
        acceleration.value = transform.rotation * Vec3::Y * ACCELERATION;
    } else { // Stop accelerating
        acceleration.value = Vec3::ZERO;
    }

    // Accelerate rotation 
    if input.pressed(ROTATE_LEFT_KEY) {
        angular_acceleration.value = ANGULAR_ACCELERATION;
    } else if input.pressed(ROTATE_RIGHT_KEY) {
        angular_acceleration.value = -ANGULAR_ACCELERATION;
    } else { // Stop Rotation Acceleration
        angular_acceleration.value = 0.0;
    }
    
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // Spawn Player
    let player_sprite = asset_server.load("sprites/rocket.png");
    commands.spawn((
        Name::from("Player"),
        MovingObjectBundle {
            sprite: SpriteBundle {
                texture: player_sprite,
                transform: Transform {
                    scale: PLAYER_SCALE,
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    ..default()
                },
                ..default()
            },
            // TODO: Get each max working
            velocity: Velocity::default(),
            acceleration: Acceleration::default().with_max(0.01),
            angular_velocity: AngularVelocity::default(),
            angular_acceleration: AngularAcceleration::default(),
        },
        Player,
    ));
}

pub fn despawn_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    for entity in player_query.iter() {
        commands.entity(entity).despawn();
    }
}
