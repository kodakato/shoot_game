mod components;
mod resources;
mod systems;

use bevy::prelude::*;

use resources::*;

use crate::AppState;

use self::systems::*;

use super::{movement::ACCELERATION, SimulationState};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
        .add_systems(OnTransition{from: AppState::MainMenu, to: AppState::InGame},spawn_enemy)
        .add_systems(OnTransition{from: AppState::InGame, to: AppState::MainMenu}, despawn_enemies)
        .add_systems(Update, move_to_player.run_if(in_state(AppState::InGame)));
    }
}

pub const ENEMY_ACCELERATION: f32 = ACCELERATION * 0.2;
pub const ALERT_DISTANCE: f32 = 400.0;