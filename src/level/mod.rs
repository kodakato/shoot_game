mod wall;

use bevy::prelude::*;
use wall::WallPlugin;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WallPlugin);
    }
}