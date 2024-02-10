use bevy::prelude::*;
use crate::game::{movement::components::{MovingObjectBundle, Velocity}, Health};

use super::{PROJECTILE_DAMAGE, PROJECTILE_LIFETIME, PROJECTILE_SPEED, STARTING_HEALTH};


#[derive(Component)]
pub struct Player;


#[derive(Component)]
pub struct Projectile {
    pub damage: i32,
}

impl Projectile {
    pub fn shoot(direction: Vec3) -> Projectile {
        Projectile {
            damage: PROJECTILE_DAMAGE,
        }
    }
}

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