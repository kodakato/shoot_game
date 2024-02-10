
use bevy::prelude::*;
use bevy_audio::AudioPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod main_menu;
mod game;
mod systems;

use main_menu::MainMenuPlugin;
use game::GamePlugin;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Update, exit_game)
        .add_systems(Update, (transition_to_game_state, transition_to_main_menu_state))
        .run();
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}





