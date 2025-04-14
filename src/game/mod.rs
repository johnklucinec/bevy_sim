/// Author: John Klucinec (@johnklucinec)
mod biome;
mod camera;
pub mod car;
pub mod terrain;
pub mod python;
mod systems;
pub mod ui;

use crate::game::car::input::*;

use crate::game::biome::BiomePlugin;
use crate::game::car::CarPlugin;
use crate::game::terrain::TerrainPlugin;
use crate::game::systems::pause_simulation;
use crate::game::systems::resume_simulation;
use crate::AppState;
use camera::SecondaryCameraPlugin;
use python::PythonPlugin;
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
            .init_resource::<CarInput>()
            // On Enter Systems
            .add_systems(OnEnter(AppState::Game), resume_simulation)
            // Plugins
            .add_plugins((
                GameUIPlugin,
                SecondaryCameraPlugin,
                CarPlugin,
                TerrainPlugin,
                BiomePlugin,
                PythonPlugin,
            ))
            // Systems
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            // On Exit Systems
            .add_systems(OnExit(AppState::Game), pause_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Paused,
    Running,
}
