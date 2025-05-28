/// Author: John Klucinec (@johnklucinec)
mod pause_menu;
mod speedometer;
mod free_cam_text;

use bevy::prelude::*;
use pause_menu::PauseMenuPlugin;
use speedometer::SpeedometerPlugin;
use free_cam_text::FreeCamTextPlugin;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            // States
            .init_state::<HUDOverlayState>()
            .add_plugins((PauseMenuPlugin, SpeedometerPlugin, FreeCamTextPlugin));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum HUDOverlayState {
    #[default]
    Visible,
    Hidden,
}
