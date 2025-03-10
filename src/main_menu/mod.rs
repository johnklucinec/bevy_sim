mod components;
mod styles;
mod systems;

use crate::AppState;
use bevy::prelude::*;
use systems::{
    interactions::{
        interact_with_disabled_button, interact_with_play_button, interact_with_quit_button,
    },
    layout::*,
};

/// Bevy plugin responsible for managing the main menu state.
///
/// # Functionality
///
/// * Spawning and despawning of the main menu
/// * Interactions with the play and quit buttons
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            // Systems
            .add_systems(
                Update,
                (
                    interact_with_play_button,
                    interact_with_quit_button,
                    interact_with_disabled_button, // Used for testing/buttons with no functionality
                ),
            )
            // OnExit State Systems
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
