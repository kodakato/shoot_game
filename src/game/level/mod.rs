pub mod wall;
pub mod systems;

use bevy::prelude::*;
use wall::WallPlugin;
use systems::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_level)
        .add_plugins(WallPlugin);
    }
}