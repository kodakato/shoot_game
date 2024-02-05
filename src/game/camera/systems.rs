use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn((Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    },
    Camera,
    Name::from("Camera"),
));
}

// pub fn centre_on_player(
//     mut camera_query: Query<&mut Transform, With<Camera>>,
//     player_query: Query<&Transform, With<Player>>,
// ) {
//     if let Ok(player_transform) = player_query.get_single() {
//         if let Ok(mut camera_transform) = camera_query.get_single_mut() {
//             camera_transform.translation.x = player_transform.translation.x;
//             camera_transform.translation.y = player_transform.translation.y;
//         }
//     }
// }