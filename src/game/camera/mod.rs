use bevy::prelude::*;
use components::SecondaryCameraState;
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
            .add_systems(OnEnter(AppState::Game), systems::spawn_secondary_camera)
            .add_plugins(CameraViewUiPlugin)
            .add_systems(
                Update,
                (
                    toggle_secondary_camera,
                    update_car_camera
                        .after(systems::spawn_secondary_camera)
                        .run_if(in_state(AppState::Game)),
                ),
            )
            .add_systems(OnExit(AppState::Game), despawn_secondary_camera);
    }
}
