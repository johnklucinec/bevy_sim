use super::components::{CommandMessage, CommandQueue};
use bevy::prelude::*;

// Types of Commands that can be sent to the python script
pub enum CommandType {
    Detect,
    Reset,
}

// Implementation of the commands
impl CommandType {
    pub fn to_string(&self) -> &'static str {
        match self {
            CommandType::Detect => "DETECT",
            CommandType::Reset => "RESET",
        }
    }
}

// Keyboard Commands
pub fn queue_commands(mut commands: ResMut<CommandQueue>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        commands.enqueue(CommandMessage::new(CommandType::Detect, "traffic cone"));
    }

    if input.just_pressed(KeyCode::KeyR) {
        commands.enqueue(CommandMessage::new(CommandType::Reset, String::new()));
    }
}
