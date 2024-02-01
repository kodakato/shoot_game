pub mod components;
pub mod systems;

use bevy::prelude::*;

use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement)
        .add_systems(Startup, spawn_player);
    }
}
