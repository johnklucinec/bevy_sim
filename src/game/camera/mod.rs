use bevy::prelude::*;
use components::{PythonProcess, SecondaryCameraState, SecondaryWindow};
use systems::update_car_camera;
pub use systems::{
    despawn_secondary_camera, toggle_secondary_camera, VIEWPORT_POSITION, VIEWPORT_SIZE,
};

use crate::AppState;

use super::ui::CameraViewUiPlugin;
pub mod components;
mod systems;

pub struct SecondaryCameraPlugin;

impl Plugin for SecondaryCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SecondaryCameraState>()
            // Spawn but dont enable the camera
            .add_systems(OnEnter(AppState::Game), systems::spawn_secondary_camera)
            // Add the UI for the camera
            .add_plugins(CameraViewUiPlugin)
            // Holds the PID for the python script + window
            .insert_resource(PythonProcess(None))
            .insert_resource(SecondaryWindow(Entity::from_raw(0)))
            // Camera Systems
            .add_systems(Update, toggle_secondary_camera)
            .add_systems(
                Update,
                update_car_camera
                    .after(systems::spawn_secondary_camera)
                    .run_if(in_state(AppState::Game)),
            )
            // Remove camera on main menu
            .add_systems(OnExit(AppState::Game), despawn_secondary_camera);
    }
}
