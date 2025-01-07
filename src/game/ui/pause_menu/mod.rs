mod components;
mod styles;
mod systems;

use systems::interactions::*;
use systems::layout::*;

use crate::game::SimulationState;
use crate::AppState;
use bevy::prelude::*;

/// Bevy plugin responsible for managing the pause menu state.
///
/// # Functionality
///
/// * Spawning and despawning of the pause menu
/// * Interactions with the resume, main menu, and quit buttons
pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(
                OnEnter(SimulationState::Paused),
                spawn_pause_menu.run_if(in_state(AppState::Game)), // Ensure this only runs the when App state is in Game
            )
            // Systems
            .add_systems(
                Update,
                (
                    interact_with_resume_button,
                    interact_with_main_menu_button,
                    interact_with_quit_button,
                    interact_with_hud_button,
                    interact_with_disabled_button,
                ),
            )
            // OnExit State Systems
            .add_systems(OnExit(SimulationState::Paused), despawn_pause_menu)
            .add_systems(OnExit(AppState::Game), despawn_pause_menu);
    }
}
