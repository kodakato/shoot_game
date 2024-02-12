use bevy::prelude::*;

use crate::game::{collision::components::CollisionEvent, enemy::components::Enemy, Health};
use crate::game::player::components::LifeTimer;
use super::components::Projectile;


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