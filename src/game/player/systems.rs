use bevy::ecs::entity;
use bevy::prelude::*;
use crate::game::collision::components::{Collider, CollisionEvent};
use crate::game::enemy::components::Enemy;
use crate::game::{movement::*, Health};

use crate::game::movement::components::{Acceleration, AngularAcceleration, AngularVelocity, Velocity};
use crate::game::movement::components::MovingObjectBundle;

use super::components::{LifeTimer, Player, Projectile};
use super::*;

// This function only sets the values for the player's movement. The actual movement is handled in the movement system.
pub fn player_movement(
    mut player_query: Query<(&mut Acceleration, &Transform, &mut AngularAcceleration), With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    if let Ok((mut acceleration, transform, mut angular_acceleration)) = player_query.get_single_mut() {
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
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // Spawn Player
    let player_sprite = asset_server.load("sprites/ship.png");
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
            collider: Collider::new(PLAYER_COLLIDER_SIZE / 2.0),
        },
        Player,
        Health::new(STARTING_HEALTH),
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
    asset_server: Res<AssetServer>,
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
                collider: Collider::new(PROJECTILE_SIZE.y / 2.0),
            },
            Projectile::shoot(direction),
            LifeTimer::new(PROJECTILE_LIFETIME),
        ));

        // Play sound
        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/shoot.ogg"),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

// Despawn any projectiles that either exist for too long, or are too far from the player
pub fn despawn_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut projectile_query: Query<(Entity, &mut LifeTimer), With<Projectile>>,
) {
    for (entity, mut life_timer) in projectile_query.iter_mut() {
        // Update the timer
        if life_timer.timer.tick(time.delta()).finished() {
            commands.entity(entity).despawn_recursive();
        } 
    }
}

pub fn projectile_touches_enemy(
    mut collision_events: EventReader<CollisionEvent>,
    mut enemy_health_query: Query<&mut Health, With<Enemy>>,
    projectile_query: Query<(), With<Projectile>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for collision_event in collision_events.iter() {
        let CollisionEvent {
            entity,
            collided_entity,
        } = collision_event;

        // Check if one of the entities is a projectile and the other is an enemy
        let is_projectile_and_enemy = (projectile_query.get(*entity).is_ok() && enemy_health_query.get_mut(*collided_entity).is_ok()) ||
                                      (projectile_query.get(*collided_entity).is_ok() && enemy_health_query.get_mut(*entity).is_ok());

        // If it's a collision between a projectile and an enemy, reduce health
        if is_projectile_and_enemy {
            let mut hp = 0;
            if let Ok(mut health) = enemy_health_query.get_mut(*collided_entity) {
                health.amount -= 1;
                hp = health.amount;
            } else if let Ok(mut health) = enemy_health_query.get_mut(*entity) {
                // In case the collided_entity is the projectile and the entity is the enemy
                health.amount -= 1;
                hp = health.amount;
            }
            // Despawn the projectile
            commands.entity(*collided_entity).despawn_recursive();

            if hp > 0 {
                // Play sound
                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds/hit.ogg"),
                    settings: PlaybackSettings::DESPAWN,
                });
            }
        }
    }
}

pub fn player_killed (
    mut next_state: ResMut<NextState<AppState>>,
    query: Query<(), With<Player>>,
) {
    if query.get_single().is_err() {
        next_state.0 = Some(AppState::GameOver);
    }
}