pub mod components;
mod resources;
mod systems;

use bevy::prelude::*;

use resources::*;

use crate::AppState;

use self::systems::*;

use super::{movement::{ACCELERATION, MAX_VELOCITY}, schedule::InGameSet};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
        .insert_resource(EnemySpawnTimer::default())
        .add_systems(OnTransition{from: AppState::MainMenu, to: AppState::InGame}, spawn_one_enemy)
        .add_systems(OnTransition{from: AppState::InGame, to: AppState::MainMenu}, despawn_enemies)
        .add_systems(Update, (
            move_to_player,
            spawn_enemies,
            enemy_touches_player,
        ).in_set(InGameSet::EntityUpdates)
        .run_if(in_state(AppState::InGame)));
    }
}

pub const ENEMY_ACCELERATION: f32 = ACCELERATION;
pub const ENEMY_MAX_VELOCITY: f32 = MAX_VELOCITY * 0.1;
pub const ALERT_DISTANCE: f32 = 400.0;
pub const ENEMY_SCALE: Vec3 = Vec3::new(0.1, 0.1, 0.0);
pub const ENEMY_SPAWN_TIME: f32 = 2.0;
pub const ENEMY_COLLIDER_SIZE: f32 = 20.0;
pub const ENEMY_HEALTH: i32 = 10;
