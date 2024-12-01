use crate::game::SimulationState;

use super::components::SecondaryCamera;
use bevy::prelude::*;
use bevy::render::camera::Viewport;

pub fn spawn_secondary_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 5.0, 5.0)).looking_at(Vec3::ZERO, Vec3::Y),
        Camera {
            viewport: Some(Viewport {
                physical_position: UVec2::new(0, 0),
                physical_size: UVec2::new(300, 300),
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
) {
    if *simulation_state.get() == SimulationState::Running && input.just_pressed(KeyCode::Tab) {
        if let Ok(mut camera) = camera_query.get_single_mut() {
            camera.is_active = !camera.is_active;
        }
    }
}

pub fn disable_secondary_camera(mut camera_query: Query<&mut Camera, With<SecondaryCamera>>) {
    if let Ok(mut camera) = camera_query.get_single_mut() {
        camera.is_active = !camera.is_active;
    }
}
