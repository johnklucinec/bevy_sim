mod biome;
mod road;
mod systems;
mod ui;

use crate::game::systems::pause_simulation;
use crate::game::systems::resume_simulation;
use crate::AppState;
use systems::*;
use ui::GameUIPlugin;

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
        app
            // States
            .init_state::<SimulationState>()
            // On Enter Systems
            .add_systems(
                OnEnter(AppState::Game),
                (resume_simulation, spawn_biome_on_enter),
            )
            // Plugins
            .add_plugins(GameUIPlugin)
            // Systems
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            // On Exit Systems
            .add_systems(OnExit(AppState::Game), pause_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
