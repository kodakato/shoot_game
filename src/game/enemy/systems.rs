

use bevy::prelude::*; 

use crate::game::{movement::{components::*, MAX_VELOCITY}, player::{self, components::*}};

use super::{components::*, ALERT_DISTANCE, ENEMY_ACCELERATION};

pub fn spawn_enemy(
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
                    scale: Vec3::splat(0.1),
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            velocity: Velocity::default(),
            acceleration: Acceleration::default(),
            angular_velocity: AngularVelocity::default(),
            angular_acceleration: AngularAcceleration::default(),
        },
        Enemy,
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
    mut query: Query<(&mut Velocity, &Transform), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let player_transform = player_query.single(); 
    for (mut velocity, enemy_transform) in query.iter_mut() {
        // Only go to player if close enough
        if enemy_transform.translation.distance(player_transform.translation) > ALERT_DISTANCE {
            continue;
        }

        // Calculate the direction vector from enemy to player
        let direction_to_player = player_transform.translation - enemy_transform.translation;
        let direction_to_player_normalized = direction_to_player.normalize_or_zero();

        // Update the velocity to move towards the player
        velocity.value += direction_to_player_normalized * ENEMY_ACCELERATION * time.delta_seconds();
    }
    
}



