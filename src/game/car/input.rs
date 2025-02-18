use bevy::prelude::*;

#[derive(Resource)]
pub struct CarInput {
    pub accelerate: bool,
    pub brake: bool,
    pub turn_left: bool,
    pub turn_right: bool,
    pub toggle_gear: bool,
    pub text_command: Option<String>,
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
        }
    }
}

impl CarInput {
    pub fn parse_text_command(&mut self) {
        if let Some(command) = self.text_command.take() {
            match command.to_lowercase().trim() {
                "go" => {
                    self.accelerate = true;
                    self.brake = false;
                },
                "stop" => {
                    self.brake = true;
                    self.accelerate = false;
                },
                "left" => {
                    self.turn_left = true;
                    self.turn_right = false;
                },
                "right" => {
                    self.turn_right = true;
                    self.turn_left = false;
                },
                "gear" => {
                    self.toggle_gear = true;
                },
                _ => {
                    // Reset all inputs if command is not recognized
                    self.reset();
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
    }
}