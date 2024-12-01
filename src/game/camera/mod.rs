use bevy::prelude::*;
pub use systems::disable_secondary_camera;
use systems::toggle_secondary_camera;
mod components;
mod systems;

pub struct SecondaryCameraPlugin;

impl Plugin for SecondaryCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_secondary_camera)
            .add_systems(Update, toggle_secondary_camera);
    }
}
