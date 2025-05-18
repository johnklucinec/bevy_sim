/// Author: John Klucinec (@johnklucinec)
use super::components::{CommandMessage, CommandQueue};
use bevy::prelude::*;
use std::str::FromStr;

// Types of Commands that can be sent to the python script
#[derive(Clone, Debug)]
pub enum CommandType {
    Detect,
    Reset,
    Steer,
    Speed,
    Pidreset,
}

// Implementation of the commands
impl CommandType {
    pub fn to_string(&self) -> &'static str {
        match self {
            CommandType::Detect => "DETECT",
            CommandType::Reset => "RESET",
            CommandType::Steer => "STEER",
            CommandType::Speed => "SPEED",
            CommandType::Pidreset => "PID_RESET",
        }
    }
}

impl FromStr for CommandType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "DETECT" => Ok(CommandType::Detect),
            "RESET" => Ok(CommandType::Reset),
            "STEER" => Ok(CommandType::Steer),
            "SPEED" => Ok(CommandType::Speed),
            "RESET_PID" => Ok(CommandType::Pidreset),
            _ => Err(format!("Unknown command: {}", s)),
        }
    }
}

// Keyboard Commands
// These are not actually used in the project, but just examples for my teammates.
pub fn queue_commands(mut commands: ResMut<CommandQueue>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        commands.enqueue(CommandMessage::new(CommandType::Detect, "traffic cone"));
    }

    if input.just_pressed(KeyCode::KeyR) {
        commands.enqueue(CommandMessage::new(CommandType::Reset, String::new()));
    }
}
