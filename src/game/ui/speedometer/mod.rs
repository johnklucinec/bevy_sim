mod components;
mod styles;
mod systems;

use systems::layout::*;

use crate::AppState;
use bevy::prelude::*;

/// Bevy plugin responsible for managing the speedometer state.
///
/// # Functionality
///
/// * Spawning and despawning of the speedometer
pub struct SpeedometerPlugin;

impl Plugin for SpeedometerPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(
                OnEnter(AppState::Game),
                spawn_speedometer, // Ensure this only runs the when App state is in Game
            )
            // OnExit State Systems
            .add_systems(OnExit(AppState::Game), despawn_speedometer);
    }
}
