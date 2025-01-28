use bevy::prelude::*;
use serde::Serialize;

#[derive(Component)]
pub struct RLCamera;

#[derive(Default, Clone)]
pub struct Actions {
    pub movement: [f32; 3],
    // Add other actions as needed
}

#[derive(Default, Serialize, Clone)]
pub struct Observations {
    pub pixel_data: Vec<u8>,
    // Add other observations as needed
}

#[derive(Resource)]
pub struct RLState {
    pub is_training: bool,
}

impl Default for RLState {
    fn default() -> Self {
        Self { is_training: false }
    }
}
