use crate::game::SimulationState;

use super::components::{SecondaryCamera, SecondaryCameraState};
use bevy::prelude::*;
use bevy::render::camera::Viewport;

pub static VIEWPORT_POSITION: [u32; 2] = [5, 5]; // [x, y]
pub static VIEWPORT_SIZE: [u32; 2] = [300, 300]; // [width, height]

pub fn spawn_secondary_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 5.0, 5.0)).looking_at(Vec3::ZERO, Vec3::Y),
        Camera {
            viewport: Some(Viewport {
                physical_position: UVec2::new(VIEWPORT_POSITION[0], VIEWPORT_POSITION[1]),
                physical_size: UVec2::new(VIEWPORT_SIZE[0], VIEWPORT_SIZE[1]),
                ..default()
            }),
            order: 1,
            is_active: false,
            ..default()
        },
        SecondaryCamera,
    ));
}

pub fn toggle_secondary_camera(
    mut camera_query: Query<&mut Camera, With<SecondaryCamera>>,
    input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    camera_state: Res<State<SecondaryCameraState>>,
    mut next_camera_state: ResMut<NextState<SecondaryCameraState>>,
) {
    if *simulation_state.get() == SimulationState::Running && input.just_pressed(KeyCode::Tab) {
        if let Ok(mut camera) = camera_query.get_single_mut() {
            match *camera_state.get() {
                SecondaryCameraState::Hidden => {
                    camera.is_active = true;
                    next_camera_state.set(SecondaryCameraState::Visible);
                }
                SecondaryCameraState::Visible => {
                    camera.is_active = false;
                    next_camera_state.set(SecondaryCameraState::Hidden);
                }
            }
        }
    }
}

pub fn despawn_secondary_camera(mut camera_query: Query<&mut Camera, With<SecondaryCamera>>) {
    if let Ok(mut camera) = camera_query.get_single_mut() {
        camera.is_active = !camera.is_active;
    }
}
