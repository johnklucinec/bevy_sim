use bevy::prelude::*;
use components::{PythonProcess, SecondaryCameraState, SecondaryWindow};
use systems::update_car_camera;
pub use systems::{
    despawn_secondary_camera, toggle_secondary_camera, VIEWPORT_POSITION, VIEWPORT_SIZE,
};

use super::ui::CameraViewUiPlugin;
use crate::game::AppState;

pub mod components;
mod systems;

pub struct SecondaryCameraPlugin;

impl Plugin for SecondaryCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            // State initialization
            .init_state::<SecondaryCameraState>()
            // Resource initialization
            .insert_resource(PythonProcess(None))
            .insert_resource(SecondaryWindow(Entity::from_raw(0)))
            // Camera lifecycle systems
            .add_systems(
                OnEnter(AppState::Game),
                systems::spawn_secondary_camera, // Spawn but don't enable the camera
            )
            .add_systems(
                OnExit(AppState::Game),
                despawn_secondary_camera, // Remove camera on main menu
            )
            // Camera control systems
            .add_systems(
                Update,
                (
                    toggle_secondary_camera,
                    update_car_camera
                        .after(systems::spawn_secondary_camera)
                        .run_if(in_state(AppState::Game)),
                ),
            )
            // UI integration
            .add_plugins(CameraViewUiPlugin);
    }
}
