mod components;
mod systems;

use systems::layout::*;

use crate::AppState;
use bevy::prelude::*;

use super::HUDOverlayState;

/// Bevy plugin responsible for managing the speedometer state.
///
/// # Functionality
///
/// * Spawning and despawning of the speedometer
pub struct SpeedometerPlugin;

impl Plugin for SpeedometerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_speedometer)
            .add_systems(OnExit(HUDOverlayState::Visible), toggle_speedometer)
            .add_systems(
                Update,
                update_speedometer.run_if(in_state(HUDOverlayState::Visible)),
            )
            .add_systems(OnExit(HUDOverlayState::Hidden), toggle_speedometer)
            .add_systems(OnExit(AppState::Game), despawn_speedometer);
    }
}
