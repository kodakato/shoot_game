use bevy::prelude::*;


#[derive(Component)]
pub struct Player;


#[derive(Component)]
pub struct LifeTimer {
    pub timer: Timer,
}

impl LifeTimer {
    pub fn new(time: f32) -> LifeTimer {
        LifeTimer {
            timer: Timer::from_seconds(time, TimerMode::Once)
        }
    }
}

impl Default for LifeTimer {
    fn default() -> LifeTimer {
        LifeTimer {
            timer: Timer::from_seconds(1.0, TimerMode::Once)
        }
    }
}