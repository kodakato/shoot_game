use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod player;
mod enemy;
mod camera;
mod level;

use player::PlayerPlugin;
use enemy::EnemyPlugin;
use camera::CameraPlugin;
use level::LevelPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(LevelPlugin)
        .run();
}



