pub mod components;
pub mod systems;

use bevy::prelude::*;

use systems::*;

use crate::AppState;



pub const PLAYER_SCALE: Vec3 = Vec3::new(0.2,0.2,0.2);


pub const ROTATE_LEFT_KEY: KeyCode = KeyCode::A;
pub const ROTATE_RIGHT_KEY: KeyCode = KeyCode::D;
pub const ACCELERATE_KEY: KeyCode = KeyCode::W;
pub const SHOOT_KEY: KeyCode = KeyCode::Space;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement.run_if(in_state(AppState::InGame)))
        .add_systems(OnTransition{from: AppState::MainMenu, to: AppState::InGame}, spawn_player)
        .add_systems(OnExit(AppState::InGame), despawn_player);
    }
}
