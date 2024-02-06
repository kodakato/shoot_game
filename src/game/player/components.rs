use bevy::prelude::*;
use crate::game::{movement::components::{MovingObjectBundle, Velocity}, Health};

use super::{PROJECTILE_DAMAGE, PROJECTILE_SPEED, STARTING_HEALTH};


#[derive(Component)]
pub struct Player {
    pub health: Health,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            health: Health::new(STARTING_HEALTH),
        }
    }
}

#[derive(Component)]
pub struct Projectile {
    pub damage: i32,
    pub body: MovingObjectBundle,
}

impl Projectile {
    pub fn shoot(direction: Vec3) -> Projectile {
        Projectile {
            damage: PROJECTILE_DAMAGE,
            body: MovingObjectBundle {
                velocity: Velocity::new().with_value(direction.normalize() * PROJECTILE_SPEED),
                ..default()
            }
        }
    }
}