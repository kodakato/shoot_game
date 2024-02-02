use bevy::prelude::*;

pub const ENEMY_SPAWN_TIME: f32 = 5.0;

#[derive(Resource)]
pub struct EnemySpawnTimer(Timer);

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer(Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating))
    }
}