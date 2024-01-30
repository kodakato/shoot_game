use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .add_systems(Startup, setup_camera)
        .run();
}

#[derive(Component)]
struct MyCameraMarker;

// This resource tracks the game's score
#[derive(Resource)]
struct Scoreboard {
    score: usize,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(100.0, 200.0, 0.0),
            ..default()
        },
        MyCameraMarker,
    ));
}