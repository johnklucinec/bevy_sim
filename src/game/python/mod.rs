pub mod commands;
pub mod components;
pub mod systems;

use bevy::prelude::*;
use commands::queue_commands;
use components::{CommandQueue, PythonComms};
use systems::{handle_responses, process_command_queue};

pub struct PythonPlugin;

impl Plugin for PythonPlugin {
    fn build(&self, app: &mut App) {
        app
            // Declare but don't initialize PythonComms here
            .add_event::<components::PythonEvent>()
            .init_resource::<CommandQueue>()
            // Add systems that only run when PythonComms exists
            .add_systems(
                Update,
                (
                    queue_commands, // Queue commands from any source
                    process_command_queue.run_if(resource_exists::<PythonComms>),
                    handle_responses.run_if(resource_exists::<PythonComms>),
                ),
            );
    }
}
