use bevy::prelude::*;

mod components;
mod systems;

use bevy_rl::{AIGymPlugin, AIGymSettings, AIGymState, SimulationState};
pub use components::*;
use systems::*;

use super::camera::VIEWPORT_SIZE;

pub struct RLPlugin;

impl Plugin for RLPlugin {
    fn build(&self, app: &mut App) {
        let ai_gym_state = AIGymState::<Actions, Observations>::new(AIGymSettings {
            width: VIEWPORT_SIZE[0],
            height: VIEWPORT_SIZE[1],
            num_agents: 1,
            render_to_buffer: true,
            pause_interval: 0.01,
            ..default()
        });

        app.insert_resource(ai_gym_state)
            .add_plugins(AIGymPlugin::<Actions, Observations>::default())
            .add_systems(Startup, check_and_setup_camera)
            .add_systems(
                Update,
                (handle_pause, process_actions).in_set(SimulationState::PausedForControl),
            );
    }
}
