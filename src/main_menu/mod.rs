use crate::AppState;
use bevy::prelude::*;

pub struct MainMenuPlugin;

pub mod components;
pub mod interactions;
pub mod layout;

use interactions::*;
use layout::*;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
            .add_systems(
                Update,
                (interact_with_play_button, interact_with_quit_button)
                    .run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(
                OnTransition {
                    from: AppState::MainMenu,
                    to: AppState::InGame,
                },
                despawn_main_menu,
            );
    }
}
