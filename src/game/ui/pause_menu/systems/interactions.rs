use bevy::app::AppExit;
use bevy::prelude::*;

use crate::game::ui::pause_menu::components::*;
use crate::game::ui::pause_menu::styles::*;
use crate::game::SimulationState;
use crate::AppState;

pub fn interact_with_resume_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResumeButton>),
    >,
    mut next_state: ResMut<NextState<SimulationState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();
                next_state.set(SimulationState::Running);
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

pub fn interact_with_main_menu_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();
                next_state.set(AppState::MainMenu);
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
