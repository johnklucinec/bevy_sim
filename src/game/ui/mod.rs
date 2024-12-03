pub mod camera_view;
mod pause_menu;
mod speedometer;

use bevy::prelude::*;
pub use camera_view::CameraViewUiPlugin;
use pause_menu::PauseMenuPlugin;
use speedometer::SpeedometerPlugin;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PauseMenuPlugin, SpeedometerPlugin));
    }
}
