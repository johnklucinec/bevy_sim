mod biome;
mod systems;
mod ui;

<<<<<<< HEAD
=======
use crate::game::systems::pause_simulation;
use crate::game::systems::resume_simulation;
use crate::game::systems::spawn_biome_on_enter;
>>>>>>> 1bbe529 (Added simple road to the current enviornment, removed biomes folder, commented out circle base.)
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
<<<<<<< HEAD
            .add_systems(OnEnter(AppState::Game), resume_simulation)
            // Plugins
            .add_plugins(GameUIPlugin)
            // Systems
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
=======
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            .add_systems(OnEnter(AppState::Game), spawn_biome_on_enter)
>>>>>>> 1bbe529 (Added simple road to the current enviornment, removed biomes folder, commented out circle base.)
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
