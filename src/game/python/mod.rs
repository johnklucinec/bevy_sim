pub mod components;
pub mod systems;

use bevy::prelude::*;
use components::PythonComms;
use systems::{handle_responses, send_commands};

pub struct PythonPlugin;

impl Plugin for PythonPlugin {
    fn build(&self, app: &mut App) {
        app
            // Declare but don't initialize PythonComms here
            .add_event::<components::PythonEvent>()
            // Add systems that only run when PythonComms exists
            .add_systems(
                Update,
                (
                    send_commands.run_if(resource_exists::<PythonComms>),
                    handle_responses.run_if(resource_exists::<PythonComms>),
                ),
            );
    }
}
