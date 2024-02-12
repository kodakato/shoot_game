

use std::ops::Range;

use bevy::prelude::*;
use rand::Rng; 

use crate::game::{collision::{self, components::{Collider, CollisionEvent}}, movement::{components::*, ACCELERATION, MAX_VELOCITY}, player::{self, components::*}, Health};

use super::{components::*, EnemySpawnTimer, ALERT_DISTANCE, ENEMY_ACCELERATION, ENEMY_COLLIDER_SIZE, ENEMY_HEALTH, ENEMY_MAX_VELOCITY, ENEMY_SCALE};

const SPAWN_X_RANGE: Range<f32> = -500.0..500.0;

pub fn spawn_one_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Spawn enemy
    let texture = asset_server.load("sprites/alien.png");
    commands.spawn((
        Name::from("Enemy"),
        MovingObjectBundle {
            sprite: SpriteBundle {
                texture: texture,
                transform: Transform {
                    scale: ENEMY_SCALE,
                    translation: Vec3::new(0.0, 25.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            velocity: Velocity::new().with_max(ENEMY_MAX_VELOCITY),
            acceleration: Acceleration::new().with_max(ENEMY_ACCELERATION),
            angular_velocity: AngularVelocity::default(),
            angular_acceleration: AngularAcceleration::default(),
            collider: Collider::new(ENEMY_COLLIDER_SIZE / 2.0),
        },
       
    ));
}

pub fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    let translation = Vec3::new(rng.gen_range(SPAWN_X_RANGE), 25.0, 0.0);

    // Spawn enemy
    let texture = asset_server.load("sprites/alien.png");
    commands.spawn((
        Name::from("Enemy"),
        MovingObjectBundle {
            sprite: SpriteBundle {
                texture: texture,
                transform: Transform {
                    scale: ENEMY_SCALE,
                    translation,
                    ..Default::default()
                },
                ..Default::default()
            },
            velocity: Velocity::new().with_max(ENEMY_MAX_VELOCITY),
            acceleration: Acceleration::new().with_max(ENEMY_ACCELERATION),
            angular_velocity: AngularVelocity::default(),
            angular_acceleration: AngularAcceleration::default(),
            collider: Collider::new(20.0),
        },
        Enemy,
        Health::new(ENEMY_HEALTH),
    ));
}

pub fn despawn_enemies(
    mut commands: Commands,
    player_query: Query<Entity, With<Enemy>>,
) {
    for entity in player_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn move_to_player(
    mut query: Query<(&mut Acceleration, &Transform), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_query.get_single(){
        for (mut acceleration, enemy_transform) in query.iter_mut() {
            // Only go to player if close enough
            if enemy_transform.translation.distance(player_transform.translation) > ALERT_DISTANCE {
                continue;
            }

            // Calculate the direction vector from enemy to player
            let direction_to_player = player_transform.translation - enemy_transform.translation;
            let direction_to_player_normalized = direction_to_player.normalize_or_zero();

            // Update the velocity to move towards the player
            acceleration.value = direction_to_player_normalized * ENEMY_ACCELERATION;
        }
    }
}

pub fn enemy_touches_player (
    mut collision_events: EventReader<CollisionEvent>,
    mut player_health_query: Query<&mut Health, With<Player>>,
) {
    for &CollisionEvent{
        entity,
        collided_entity,
    } in collision_events.read() {
        if let Ok(mut health) = player_health_query.get_mut(collided_entity) {
            health.amount -= 1;
        }
    }
}
