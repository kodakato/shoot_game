use bevy::prelude::*;

mod player;
mod camera;
mod level;
mod enemy;
mod systems;
mod movement;
pub mod components;


use components::*;
use systems::*;
use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
        .add_plugins(player::PlayerPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(level::LevelPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(movement::MovementPlugin)
        .add_systems(OnEnter(AppState::InGame), toggle_simulation);
    }
}

#[derive(Component)]
struct CollisionBox {
    width: f32,
    height: f32,
}

