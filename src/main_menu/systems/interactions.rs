use bevy::prelude::*;

use crate::{
    main_menu::{
        components::*,
        styles::{
            HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON, SECONDARY_BUTTON,
            SECONDARY_HOVERED_BUTTON, SECONDARY_PRESSED_BUTTON,
        },
    },
    AppState,
};

// Gives funcationality to the Play Button
pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();
                next_state.set(AppState::Game);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON.into();
            }
        }
    }
}

// Gives functionality to the Quit Button
pub fn interact_with_quit_button(
    mut app_exit: EventWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = SECONDARY_PRESSED_BUTTON.into();
                app_exit.send(AppExit::Success);
            }
            Interaction::Hovered => {
                *background_color = SECONDARY_HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *background_color = SECONDARY_BUTTON.into();
            }
        }
    }
}

// Used for when a button is disabled
// Logs a message when a disabled button is pressed
pub fn interact_with_disabled_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<DisabledButton>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();
                println!("Button is currently disabled");
            }
            Interaction::Hovered => {
                *background_color = PRESSED_BUTTON.into();
            }
            Interaction::None => {
                *background_color = PRESSED_BUTTON.into();
            }
        }
    }
}
