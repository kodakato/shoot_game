pub mod components;
pub mod systems;

use bevy::prelude::*;
use systems::*;
pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
    }
}