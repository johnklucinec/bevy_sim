use bevy::prelude::*;

use crate::game::SimulationState;

/// Pauses the simulation when entering the game state.
pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Paused);
}

/// Resumes the simulation when exiting the game state.
pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Running);
}
