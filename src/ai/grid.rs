use bevy::prelude::*;

pub const GRID_SIZE: usize = 10;

#[derive(Resource)]
pub struct Grid {
    pub entities: [[Option<Entity>; GRID_SIZE]; GRID_SIZE],
}

impl Grid {
    pub fn new() -> Self {
        Self {
            entities: [[None; GRID_SIZE]; GRID_SIZE],
        }
    }
}
