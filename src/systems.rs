const START_GAME_KEY: KeyCode = KeyCode::Space;
const EXIT_GAME_KEY: KeyCode = KeyCode::Escape;
const MAIN_MENU_KEY: KeyCode = KeyCode::M;

use bevy::prelude::*;
use bevy::app::AppExit;
use crate::AppState;

pub fn transition_to_game_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(START_GAME_KEY) {
        if *app_state.get() != AppState::InGame {
            next_app_state.set(AppState::InGame);
            println!("Transitioning to InGame state");
        }
    }
}

pub fn transition_to_main_menu_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>
) {
    if keyboard_input.just_pressed(MAIN_MENU_KEY) {
        if *app_state.get() != AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
            println!("Transitioning to MainMenu state");
        }
    }
}

pub fn exit_game (
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(EXIT_GAME_KEY) {
        app_exit_event_writer.send(AppExit);
    }
}