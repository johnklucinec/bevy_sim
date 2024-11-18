mod biome;
mod systems;

use crate::game::systems::pause_simulation;
use crate::game::systems::resume_simulation;
use crate::game::systems::spawn_biome_on_enter;
use crate::AppState;

use bevy::prelude::*;

/// Bevy plugin responsible for managing the game's simulation state.
///
/// # Functionality
///
/// * Pausing the simulation when entering the game state
/// * Resuming the simulation when exiting the game state.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            // On Enter Systems
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            .add_systems(OnEnter(AppState::Game), spawn_biome_on_enter)
            // On Exit Systems
            .add_systems(OnExit(AppState::Game), resume_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
