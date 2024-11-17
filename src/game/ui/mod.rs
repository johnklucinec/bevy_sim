mod pause_menu;

use bevy::prelude::*;
use pause_menu::PauseMenuPlugin;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PauseMenuPlugin);
    }
}
