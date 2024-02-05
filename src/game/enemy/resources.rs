use bevy::prelude::*;

use super::components::*;

#[derive(Resource)]
pub struct EnemySpawnTimer(Timer);

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer(Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating))
    }
}