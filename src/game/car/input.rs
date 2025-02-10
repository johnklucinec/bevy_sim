use bevy::prelude::*;

#[derive(Resource)]
pub struct CarInput {
    pub accelerate: bool,
    pub brake: bool,
    pub turn_left: bool,
    pub turn_right: bool,
    pub toggle_gear: bool,
}

impl Default for CarInput {
    fn default() -> Self {
        Self {
            accelerate: false,
            brake: false,
            turn_left: false,
            turn_right: false,
            toggle_gear: false,
        }
    }
}