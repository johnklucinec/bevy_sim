use bevy::app::AppExit;
use bevy::prelude::*;

use crate::game::ui::pause_menu::components::*;
use crate::game::ui::pause_menu::styles::*;
use crate::game::ui::HUDOverlayState;
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
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = SECONDARY_PRESSED_BUTTON.into();
                next_state.set(AppState::MainMenu);
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

fn toggle_hud_state(
    hud_state: Res<State<HUDOverlayState>>,
    mut next_hud_state: ResMut<NextState<HUDOverlayState>>,
) {
    match *hud_state.get() {
        HUDOverlayState::Hidden => next_hud_state.set(HUDOverlayState::Visible),
        HUDOverlayState::Visible => next_hud_state.set(HUDOverlayState::Hidden),
    }
}

// Used for when a button is disabled
pub fn interact_with_hud_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<HUDToggle>),
    >,
    hud_state: Res<State<HUDOverlayState>>,
    next_state: ResMut<NextState<HUDOverlayState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();
                toggle_hud_state(hud_state, next_state);
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
