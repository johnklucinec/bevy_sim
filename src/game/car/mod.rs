use bevy::prelude::*;

pub mod car;
pub mod physics;
pub mod input;

use crate::game::AppState;
use crate::spawn_car;
use crate::move_car;
use crate::reset_car;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app
            // Spawning the car
            .add_systems(OnEnter(AppState::Game), spawn_car)
            // Hide Camera View UI when exiting visible camera state
            .add_systems(Update,(move_car, reset_car).run_if(in_state(AppState::Game)));
    }
}
