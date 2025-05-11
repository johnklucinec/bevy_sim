use bevy::{prelude::*, window::exit_on_primary_closed};
use components::{SecondaryCameraState, SecondaryWindow};
use systems::{
    cleanup_python_comms, cleanup_system, despawn_secondary_camera, kill_python_process,
    spawn_python_process, spawn_secondary_camera, toggle_secondary_camera, update_car_camera,
};

use crate::game::AppState;

pub mod components;
pub mod systems;

pub struct SecondaryCameraPlugin;

impl Plugin for SecondaryCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            // State initialization
            .init_state::<SecondaryCameraState>()
            .insert_resource(SecondaryWindow(Entity::from_raw(0)))
            // Camera lifecycle systems
            .add_systems(
                OnEnter(AppState::Game),
                spawn_secondary_camera, // Spawn but don't enable the camera
            )
            .add_systems(
                OnExit(AppState::Game),
                (
                    cleanup_python_comms,
                    kill_python_process,
                    despawn_secondary_camera,
                ),
            )
            .add_systems(OnEnter(SecondaryCameraState::Visible), spawn_python_process)
            .add_systems(OnExit(SecondaryCameraState::Visible), kill_python_process)
            // Camera control systems
            .add_systems(PostUpdate, cleanup_system)
            .add_systems(
                Update,
                (
                    toggle_secondary_camera,
                    exit_on_primary_closed,
                    update_car_camera.run_if(in_state(SecondaryCameraState::Visible)),
                ),
            );
    }
}
