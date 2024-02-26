use bevy::{app::AppExit, prelude::*};

use crate::main_menu::components::{PlayButton, QuitButton};
use crate::AppState;

pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor(Color::rgb(0.0, 0.0, 1.0));
                next_app_state.set(AppState::InGame);
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::rgb(0.0, 0.0, 0.5));
            }
            Interaction::None => {
                *background_color = BackgroundColor(Color::rgb(0.0, 0.0, 0.0));
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor(Color::rgb(0.0, 0.0, 1.0));
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::rgb(0.0, 0.0, 0.5));
            }
            Interaction::None => {
                *background_color = BackgroundColor(Color::rgb(0.0, 0.0, 0.0));
            }
        }
    }
}

