use crate::game::python::commands::CommandType;
use crate::game::python::components::{CommandEvent, CommandMessage, CommandQueue};
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CarInput {
    pub accelerate: bool,
    pub brake: bool,
    pub turn_left: bool,
    pub turn_right: bool,
    pub toggle_gear: bool,
    pub text_command: Option<String>,
    // New continuous control values
    pub speed_value: f32,
    pub steer_angle: f32,
}

impl CarInput {
    pub fn parse_text_command(&mut self) {
        if let Some(command) = self.text_command.take() {
            // Create a longer-lived value by storing the lowercase string
            let lowercase_command = command.to_lowercase();
            let parts: Vec<&str> = lowercase_command.trim().split_whitespace().collect();

            match parts.as_slice() {
                ["speed", value_str] => {
                    if let Ok(value) = value_str.parse::<f32>() {
                        self.speed_value = value;
                        // Clear direct control inputs since we're using continuous values
                        self.accelerate = false;
                        self.brake = false;
                    } else {
                        println!("Invalid speed value: {}", value_str);
                    }
                }
                ["steer", value_str] => {
                    if let Ok(mut value) = value_str.parse::<f32>() {
                        // limit to 30 degrees
                        if value > 30.0 {
                            value = 30.0;
                        } else if value < -30.0 {
                            value = -30.0;
                        }

                        self.steer_angle = value;
                        // Clear direct control inputs since we're using continuous values
                        self.turn_left = false;
                        self.turn_right = false;
                    } else {
                        println!("Invalid turn angle value: {}", value_str);
                    }
                }
                _ => {
                    // Reset all inputs if command is not recognized
                    // But keep continuous values
                    self.accelerate = false;
                    self.brake = false;
                    self.turn_left = false;
                    self.turn_right = false;
                    self.toggle_gear = false;
                }
            }
        }
    }
}

// Keybindings to manually send commands to the Python script
// This is for proof of concept testing - will be replaced by AI-driven commands later
pub fn car_commands(mut commands: ResMut<CommandQueue>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Digit1) {
        commands.enqueue(CommandMessage::new(CommandType::Speed, "10"));
    }

    if input.just_pressed(KeyCode::Digit2) {
        commands.enqueue(CommandMessage::new(CommandType::Speed, "50"));
    }

    if input.just_pressed(KeyCode::Digit3) {
        commands.enqueue(CommandMessage::new(CommandType::Speed, "0"));
    }

    if input.just_pressed(KeyCode::Digit4) {
        commands.enqueue(CommandMessage::new(CommandType::Steer, "10"));
    }

    if input.just_pressed(KeyCode::Digit5) {
        commands.enqueue(CommandMessage::new(CommandType::Steer, "-50"));
    }

    if input.just_pressed(KeyCode::Digit6) {
        commands.enqueue(CommandMessage::new(CommandType::Steer, "0"));
    }
}

pub fn handle_car_commands(mut event_reader: EventReader<CommandEvent>) {
    for event in event_reader.read() {
        match (&event.command_type, event.value) {
            //Process STEER Commands
            (CommandType::Steer, value) => match value {
                // TODO: Set the car steering angle
                Some(num) => println!("Steering updated to: {}", num),
                None => println!("Error: {}", event.string_value),
            },

            //Process SPEED Commands
            (CommandType::Speed, value) => match value {
                // TODO: Set the car SPEED
                Some(num) => println!("Car speed updated to: {}", num),
                None => println!("Error: {}", event.string_value),
            },

            // Ignore other commands
            _ => (),
        }
    }
}
