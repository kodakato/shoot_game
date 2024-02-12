use bevy::prelude::*;

use crate::game::player::PROJECTILE_DAMAGE;

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