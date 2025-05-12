use crate::game::car::car::*;
use crate::game::car::input::*;
use crate::game::car::physics::*;
use bevy::prelude::*;
use car::despawn_cars;

pub mod car;
pub mod input;
pub mod physics;

use crate::game::AppState;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app
            // Spawning the car
            .add_systems(OnEnter(AppState::Game), spawn_car)
            // Hide Camera View UI when exiting visible camera state
            .add_systems(
                FixedUpdate,
                (
                    move_car, 
                    reset_car, 
                    car_commands, 
                    handle_car_commands,
                )
                    .run_if(in_state(AppState::Game)),
            )
            // Add toggle_driving_state to the Update schedule
            .add_systems(Update, toggle_driving_state.run_if(in_state(AppState::Game)))
            // Spawning the car
            .add_systems(OnExit(AppState::Game), despawn_cars);
    }
}


#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum DrivingState {
    #[default]
    Autonomous,
    Manual,
}
