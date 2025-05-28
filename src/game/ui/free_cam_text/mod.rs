/// Author: John Klucinec (@johnklucinec)
mod components;
mod systems;

use systems::layout::*;

use crate::AppState;
use bevy::prelude::*;

use super::HUDOverlayState;

/// Bevy plugin responsible for managing the free camera text state.
///
/// # Functionality
///
/// * Spawning and despawning of the free camera text
pub struct FreeCamTextPlugin;

impl Plugin for FreeCamTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_camera_text)
            .add_systems(OnExit(HUDOverlayState::Visible), toggle_camera_text)
            .add_systems(
                Update,
                update_camera_text.run_if(in_state(HUDOverlayState::Visible)),
            )
            .add_systems(OnExit(HUDOverlayState::Hidden), toggle_camera_text)
            .add_systems(OnExit(AppState::Game), despawn_camera_text);
    }
}
