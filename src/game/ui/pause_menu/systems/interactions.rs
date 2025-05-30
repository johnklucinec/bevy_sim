/// Author: John Klucinec (@johnklucinec)
use bevy::{app::AppExit, prelude::*};
use crate::game::{
    camera::components::SecondaryCameraState,
    car::{car::Car, input::CarInput, physics::reset_car_to_spawn},
    ui::{pause_menu::{components::*, styles::*}, HUDOverlayState},
    SimulationState,
};
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

pub fn interact_with_reset_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResetButton>),
    >,
    mut car_query: Query<(&mut Car, &mut Transform), With<Car>>,
    mut car_input: ResMut<CarInput>,
    camera_state: Res<State<SecondaryCameraState>>,
    mut next_state: ResMut<NextState<SimulationState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();

                // Reset the car
                if let Ok((mut car, mut transform, )) = car_query.get_single_mut() {
                    reset_car_to_spawn(&mut transform, &mut car, &mut car_input, camera_state);
                }

                // Close pause menu
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
