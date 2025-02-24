
use crate::game::SimulationState;
use bevy::prelude::*;

/// Pauses the simulation when entering the paused simulation state.
pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    println!("Simulation Paused.");
    simulation_state_next_state.set(SimulationState::Paused);
}

/// Resumes the simulation when exiting the running simulation state.
pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    println!("Simulation Running.");
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match simulation_state.get() {
            SimulationState::Running => {
                pause_simulation(simulation_state_next_state);
            }
            SimulationState::Paused => {
                resume_simulation(simulation_state_next_state);
            }
        }
    }
}

