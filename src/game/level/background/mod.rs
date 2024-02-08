use bevy::prelude::*;
use bevy_tiling_background::{TilingBackgroundPlugin, BackgroundMaterial};

use self::systems::setup_background;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
       // app.add_systems(Startup,setup_background)
        //.add_plugins(TilingBackgroundPlugin::<BackgroundMaterial>::default());
    }
}

mod systems;