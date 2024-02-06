pub mod components;
pub mod systems;
pub mod resources;

use bevy::prelude::*;

use systems::*;

use crate::AppState;


pub const ROTATE_LEFT_KEY: KeyCode = KeyCode::A;
pub const ROTATE_RIGHT_KEY: KeyCode = KeyCode::D;
pub const ACCELERATE_KEY: KeyCode = KeyCode::W;
pub const SHOOT_KEY: KeyCode = KeyCode::Space;

pub const PLAYER_SCALE: Vec3 = Vec3::new(0.2,0.2,0.2);
pub const STARTING_HEALTH: i32 = 100;
pub const INVULNERABLE_TIME: f32 = 2.0;

pub const PROJECTILE_DAMAGE: i32 = 10;
pub const PROJECTILE_SPEED: f32 = 500.0;
pub const PROJECTILE_SIZE: Vec3 = Vec3::new(1.0, 5.0, 0.1);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnTransition{from: AppState::MainMenu, to: AppState::InGame}, spawn_player)
        .add_systems(OnExit(AppState::InGame), despawn_player)
        .add_systems(Update, (
            player_movement,
            shoot_projectile,
        ).run_if(in_state(AppState::InGame)));
    }
}
