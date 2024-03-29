use bevy::prelude::*;

mod camera;
mod collision;
pub mod components;
mod enemy;
mod level;
mod movement;
mod player;
mod projectile;
mod schedule;
mod starfield;
mod systems;

use crate::AppState;
use components::*;
use systems::*;

use self::schedule::InGameSet;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .configure_sets(
                Update,
                (
                    InGameSet::UserInput,
                    InGameSet::EntityUpdates,
                    InGameSet::CollisionDetection,
                    InGameSet::DespawnEntities,
                )
                    .chain(),
            )
            .add_plugins(player::PlayerPlugin)
            .add_plugins(camera::CameraPlugin)
            .add_plugins(level::LevelPlugin)
            .add_plugins(enemy::EnemyPlugin)
            .add_plugins(movement::MovementPlugin)
            .add_plugins(collision::CollisionPlugin)
            .add_systems(OnEnter(AppState::InGame), toggle_simulation)
            .add_systems(
                Update,
                (despawn_dead_entities,)
                    .in_set(InGameSet::DespawnEntities)
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
