use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct MyCameraMarker;

// This resource tracks the game's score
#[derive(Resource)]
struct Scoreboard {
    score: usize,
}

fn setup(
    mut Commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Shoot Sound
    

}
