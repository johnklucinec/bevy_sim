use crate::game::ai::{Actions, Observations};
use crate::game::SimulationState;

use super::components::{CarFollowCamera, RLCamera, SecondaryCamera, SecondaryCameraState};
use crate::game::car::car::Car;

use bevy::prelude::*;
use bevy::render::camera::{RenderTarget, Viewport};
use bevy_rl::AIGymState;

pub static VIEWPORT_POSITION: [u32; 2] = [5, 5]; // [x, y]
pub static VIEWPORT_SIZE: [u32; 2] = [300, 300]; // [width, height]

pub fn spawn_secondary_camera(
    mut commands: Commands,
    ai_gym_state: Res<AIGymState<Actions, Observations>>,
) {
    let mut ai_gym_state = ai_gym_state.lock().unwrap();
    let render_image_handle = ai_gym_state.render_image_handles[0].clone();

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 5.0, 5.0)).looking_at(Vec3::ZERO, Vec3::Y),
        Camera {
            viewport: Some(Viewport {
                physical_position: UVec2::new(VIEWPORT_POSITION[0], VIEWPORT_POSITION[1]),
                physical_size: UVec2::new(VIEWPORT_SIZE[0], VIEWPORT_SIZE[1]),
                ..default()
            }),
            order: 2,
            target: RenderTarget::Image(render_image_handle),
            is_active: true,
            ..default()
        },
        SecondaryCamera,
        CarFollowCamera,
        RLCamera,
    ));
}

pub fn update_car_camera(
    car_query: Query<&Transform, With<Car>>,
    mut camera_query: Query<&mut Transform, (With<SecondaryCamera>, Without<Car>)>,
) {
    if let Ok(car_transform) = car_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            // Calculate offset based on car's rotation
            let back_offset = car_transform.back() * 10.0; // Multiply by distance behind car
            let up_offset = car_transform.up() * 5.0; // Multiply by height above car

            // Position camera behind and above car
            camera_transform.translation = car_transform.translation + back_offset + up_offset;

            // Look forward from car's position
            let target = car_transform.translation + car_transform.forward() * 10.0;
            camera_transform.look_at(target, Vec3::Y);
        } else {
            eprintln!("Error: No camera found with SecondaryCamera component");
        }
    } else {
        eprintln!("Error: No car found with Car component");
    }
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

pub fn despawn_secondary_camera(
    mut commands: Commands,
    camera_query: Query<Entity, With<SecondaryCamera>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_camera_state: ResMut<NextState<SecondaryCameraState>>,
) {
    if *simulation_state.get() != SimulationState::Running {
        if let Ok(camera_entity) = camera_query.get_single() {
            commands.entity(camera_entity).despawn_recursive();
            next_camera_state.set(SecondaryCameraState::Hidden);
        }
    }
}
