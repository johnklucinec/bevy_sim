use bevy::prelude::*;
use car::despawn_cars;

pub mod car;
pub mod input;
pub mod physics;

use crate::game::AppState;
use crate::move_car;
use crate::reset_car;
use crate::spawn_car;
use input::car_commands;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app
            // Spawning the car
            .add_systems(OnEnter(AppState::Game), spawn_car)
            // Hide Camera View UI when exiting visible camera state
            .add_systems(
                Update,
                (move_car, reset_car, car_commands).run_if(in_state(AppState::Game)),
            )
            // Spawning the car
            .add_systems(OnExit(AppState::Game), despawn_cars);
    }
}
