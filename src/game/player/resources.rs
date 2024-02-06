use bevy::prelude::*;

use super::INVULNERABLE_TIME;

#[derive(Resource)]
pub struct InvulnerabilityTimer {
    pub timer: Timer,
}

impl Default for InvulnerabilityTimer {
    fn default() -> InvulnerabilityTimer {
        InvulnerabilityTimer {
            timer: Timer::from_seconds(INVULNERABLE_TIME, TimerMode::Once)
        }
    }
}