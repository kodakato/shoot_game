use bevy::prelude::*;

use crate::main_menu::components::{PlayButton, QuitButton};

pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor(Color::rgb(0.0, 0.0, 1.0));
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
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor(Color::rgb(0.0, 0.0, 1.0));
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

