pub mod components;
mod systems;

use bevy::prelude::*;

use systems::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        //.add_systems(Update, centre_on_player.after(spawn_camera));
    }
}