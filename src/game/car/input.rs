use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CarInput {
    pub accelerate: bool,
    pub brake: bool,
    pub turn_left: bool,
    pub turn_right: bool,
    pub toggle_gear: bool,
}
