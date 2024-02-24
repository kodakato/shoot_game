use bevy::prelude::*;
use crate::AppState;

pub struct MainMenuPlugin;

pub mod layout;
pub mod components;

use layout::*;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
        .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
