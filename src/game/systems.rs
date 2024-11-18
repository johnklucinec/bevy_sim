use bevy::prelude::*;
use crate::game::biome::setup_terrain;
use crate::game::SimulationState;

/// Pauses the simulation when entering the game state.
pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Paused);
}

/// Resumes the simulation when exiting the game state.
pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn spawn_biome_on_enter(simulation_state: Res<State<SimulationState>>, mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if *simulation_state == SimulationState::Running {
        setup_terrain(&mut commands, &mut meshes, &mut materials);
    }
}