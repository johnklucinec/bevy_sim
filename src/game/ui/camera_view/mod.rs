mod components;
mod styles;
mod systems;

use crate::game::camera::components::SecondaryCameraState;
use crate::AppState;
use bevy::prelude::*;
use systems::layout::*;

/// Bevy plugin responsible for managing the camera view UI.
///
/// # Functionality
///
/// * Spawning and despawning of the camera view UI
pub struct CameraViewUiPlugin;

impl Plugin for CameraViewUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Show Camera View UI
            .add_systems(OnEnter(SecondaryCameraState::Visible), spawn_camera_view_ui)
            // Hide Camera View UI when exiting visible camera state
            .add_systems(
                OnExit(SecondaryCameraState::Visible),
                despawn_camera_view_ui,
            )
            // Hide Camera View UI when exiting the game state
            .add_systems(OnExit(AppState::Game), despawn_camera_view_ui);
    }
}
