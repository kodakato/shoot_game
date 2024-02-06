use bevy::prelude::*;
use crate::game::movement::*;

use crate::game::movement::components::{Acceleration, AngularAcceleration, AngularVelocity, Velocity};
use crate::game::movement::components::MovingObjectBundle;

use super::components::{Player, Projectile};
use super::*;

// This function only sets the values for the player's movement. The actual movement is handled in the movement system.
pub fn player_movement(
    mut player_query: Query<(&mut Acceleration, &Transform, &mut AngularAcceleration), With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    let (mut acceleration, transform, mut angular_acceleration) = player_query.single_mut();

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
            acceleration: Acceleration::default(),
            angular_velocity: AngularVelocity::default(),
            angular_acceleration: AngularAcceleration::default(),
        },
        Player::default()
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

pub fn shoot_projectile (
    mut commands: Commands,
    player_query: Query<(&Transform, &Velocity), With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    if !input.just_pressed(SHOOT_KEY) {
        return;
    }
    if let Ok((player_transform, player_velocity))= player_query.get_single() {
        // The project always has to leave the player greater than the player's velocity
        // Calculate the direction of the projectile and velocity
        let direction = player_transform.rotation * Vec3::Y;

        let velocity = player_velocity.value + direction * PROJECTILE_SPEED;
        // Spawn projectile
        commands.spawn((
            Name::from("Projectile"),
            MovingObjectBundle {
                sprite: SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.0, 100.0, 0.0),
                        ..Default::default()
                    },
                    transform: Transform {
                        scale: PROJECTILE_SIZE,
                        translation: player_transform.translation,
                        rotation: player_transform.rotation,
                    },
                    ..Default::default()
                },
                velocity: Velocity::new().with_value(direction * PROJECTILE_SPEED), // This doesn't work right
                acceleration: Acceleration::new().with_value(velocity.normalize() * PROJECTILE_ACCELERATION),
                angular_velocity: AngularVelocity::default(),
                angular_acceleration: AngularAcceleration::default(),
            },
            Projectile::shoot(direction),
        ));
    }
}

// TODO: Despawn any projectiles that either exist for too long, or are too far from the player
//pub fn despawn_projectiles 