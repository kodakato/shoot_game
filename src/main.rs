use bevy::prelude::*;

mod debug;
mod game;
mod main_menu;
mod systems;

use debug::DebugPlugin;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(DebugPlugin)
        .add_systems(Update, transition_to_main_menu_state)
        .add_systems(Startup, play_background_music)
        .run();
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}
