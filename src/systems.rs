const MAIN_MENU_KEY: KeyCode = KeyCode::Escape;

use crate::AppState;
use bevy::prelude::*;

pub fn transition_to_main_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(MAIN_MENU_KEY) {
        if *app_state.get() != AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
            println!("Transitioning to MainMenu state");
        }
    }
}

pub fn play_background_music(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/music/music.ogg"),
        settings: PlaybackSettings::LOOP,
    });
}
