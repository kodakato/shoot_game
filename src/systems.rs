const START_GAME_KEY: KeyCode = KeyCode::P;
const EXIT_GAME_KEY: KeyCode = KeyCode::Q;
const MAIN_MENU_KEY: KeyCode = KeyCode::Escape;

use bevy::prelude::*;
use bevy::app::AppExit;
use crate::AppState;

pub fn transition_to_game_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(START_GAME_KEY) {
        if *app_state.get() != AppState::InGame {
            commands.insert_resource(NextState(Some(AppState::InGame)));
            println!("Transitioning to InGame state");
        }
    }
}

pub fn transition_to_main_menu_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(MAIN_MENU_KEY) {
        if *app_state.get() != AppState::MainMenu {
            commands.insert_resource(NextState(Some(AppState::MainMenu)));
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