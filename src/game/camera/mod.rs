use bevy::prelude::*;
use components::SecondaryCameraState;
pub use systems::{
    despawn_secondary_camera, toggle_secondary_camera, VIEWPORT_POSITION, VIEWPORT_SIZE,
};

use super::ui::CameraViewUiPlugin;
pub mod components;
mod systems;

pub struct SecondaryCameraPlugin;

impl Plugin for SecondaryCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SecondaryCameraState>()
            .add_systems(Startup, systems::spawn_secondary_camera)
            .add_plugins(CameraViewUiPlugin)
            .add_systems(Update, toggle_secondary_camera);
    }
}
