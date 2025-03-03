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
    pub throttle_value: f32,
    pub turn_angle: f32,
    // New flag to explicitly reset the car
    pub reset_car: bool,
}

impl Default for CarInput {
    fn default() -> Self {
        Self {
            accelerate: false,
            brake: false,
            turn_left: false,
            turn_right: false,
            toggle_gear: false,
            text_command: None,
            throttle_value: 0.0,
            turn_angle: 0.0,
            reset_car: false,
        }
    }
}

impl CarInput {
    pub fn parse_text_command(&mut self) {
        if let Some(command) = self.text_command.take() {
            // Create a longer-lived value by storing the lowercase string
            let lowercase_command = command.to_lowercase();
            let parts: Vec<&str> = lowercase_command.trim().split_whitespace().collect();
            
            match parts.as_slice() {
                ["go"] => {
                    self.accelerate = true;
                    self.brake = false;
                },
                ["stop"] => {
                    self.brake = true;
                    self.accelerate = false;
                    self.throttle_value = 0.0;
                },
                ["left"] => {
                    self.turn_left = true;
                    self.turn_right = false;
                },
                ["right"] => {
                    self.turn_right = true;
                    self.turn_left = false;
                },
                ["gear"] => {
                    self.toggle_gear = true;
                },
                ["throttle", value_str] => {
                    if let Ok(value) = value_str.parse::<f32>() {
                        self.throttle_value = value;
                        // Clear direct control inputs since we're using continuous values
                        self.accelerate = false;
                        self.brake = false;
                    } else {
                        println!("Invalid throttle value: {}", value_str);
                    }
                },
                ["turn", value_str] => {
                    if let Ok(mut value) = value_str.parse::<f32>() {
                        // limit to 30 degrees
                        if value > 30.0 {
                            value = 30.0;
                        } else if value < -30.0 {
                            value = -30.0;
                        }
                        
                        self.turn_angle = value;
                        // Clear direct control inputs since we're using continuous values
                        self.turn_left = false;
                        self.turn_right = false;
                    } else {
                        println!("Invalid turn angle value: {}", value_str);
                    }
                },
                ["reset"] => {
                    self.reset();
                    // Set the reset_car flag to true
                    self.reset_car = true;
                },
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

    pub fn reset(&mut self) {
        self.accelerate = false;
        self.brake = false;
        self.turn_left = false;
        self.turn_right = false;
        self.toggle_gear = false;
        // Also reset continuous values
        self.throttle_value = 0.0;
        self.turn_angle = 0.0;
        // Don't reset the reset_car flag here, it will be handled by the reset_car system
    }
}