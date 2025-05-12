use crate::game::camera::components::SecondaryCameraState;
use crate::game::python::commands::CommandType;
use crate::game::python::components::{CommandEvent, CommandMessage, CommandQueue};
use bevy::prelude::*;
use super::car::Car; // Added: Import Car component
use super::DrivingState; // Added: Import DrivingState enum

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
    pub fn parse_text_command(&mut self, car_query: &Query<&Car>) {
        // Check driving state before parsing
        match car_query.get_single() {
            Ok(car) => {
                if car.driving_state == DrivingState::Manual {
                    self.text_command.take(); // Consume and discard if manual
                    return;
                }
                // If Autonomous, proceed to parse below
            }
            Err(_) => {
                // Error getting car (no car, or multiple cars)
                // Safest to consume and discard
                self.text_command.take();
                return;
            }
        }

        // If we reach here, state is Autonomous and car exists
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
    // if input.just_pressed(KeyCode::Digit1) {
    //     commands.enqueue(CommandMessage::new(CommandType::Speed, "10"));
    // }

    // if input.just_pressed(KeyCode::Digit2) {
    //     commands.enqueue(CommandMessage::new(CommandType::Speed, "50"));
    // }

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

pub fn handle_car_commands(
    car_query: Query<&Car>,
    mut event_reader: EventReader<CommandEvent>,
    mut car_input: ResMut<CarInput>,
) {
    // Attempt to get the single Car entity.
    // If there isn't exactly one Car, or if it's in Manual mode, we ignore events.
    match car_query.get_single() {
        Ok(car) => {
            if car.driving_state == DrivingState::Manual {
                // In Manual mode, ignore commands from the event queue.
                event_reader.clear(); // Consume events without processing.
                return; // Exit early.
            }
            // If Autonomous, proceed to process events below.
        }
        Err(_) => {
            // If there's no car or multiple cars, it's ambiguous.
            // For safety, let's also ignore commands in this state.
            event_reader.clear();
            return; // Exit early.
        }
    }

    // If we're here, it means there's a single car and it's in Autonomous mode.
    for event in event_reader.read() {
        match (&event.command_type, event.value) {
            (CommandType::Steer, Some(num)) => {
                car_input.steer_angle = num;
                //println!("Steering updated to: {}", num);
            }

            (CommandType::Speed, Some(num)) => {
                car_input.speed_value = num;
                //println!("Car speed updated to: {}", num);
            }
            // Ignore other commands or missing values
            _ => (),
        }
    }
}

// New system to toggle driving state
pub fn toggle_driving_state(
    mut car_query: Query<&mut Car>,
    mut car_input: ResMut<CarInput>,
    camera_state: Res<State<SecondaryCameraState>>,
) {
    // Only proceed if camera state has changed
    if !camera_state.is_changed() {
        return;
    }

    if let Ok(mut car) = car_query.get_single_mut() {
        car.driving_state = match *camera_state.get() {
            SecondaryCameraState::Hidden => {
                println!("DrivingState changed to: Manual");
                car_input.speed_value = 0.0;
                car_input.steer_angle = 0.0;
                DrivingState::Manual
            }
            SecondaryCameraState::Visible => {
                println!("DrivingState changed to: Autonomous");
                DrivingState::Autonomous
            }
        };
    }
}
