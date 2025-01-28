use bevy::render::camera::RenderTarget;
use bevy::render::primitives::Frustum;
use bevy::{prelude::*, render::view::VisibleEntities};
use bevy_rl::EventPause;
use bevy_rl::{AIGymState, EventControl};
use serde_json;

use super::{Actions, Observations, RLCamera};

pub fn setup_rl_camera(
    mut commands: Commands,
    ai_gym_state: Res<AIGymState<Actions, Observations>>,
) {
    let ai_gym_state_inner = ai_gym_state.lock().unwrap();

    if ai_gym_state_inner.render_image_handles.is_empty() {
        return; // Wait until handles are available
    }

    let render_image_handle = ai_gym_state_inner.render_image_handles[0].clone();

    commands.spawn((
        Camera3d { ..default() },
        Camera {
            target: RenderTarget::Image(render_image_handle),
            order: 3,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Projection::Perspective(PerspectiveProjection {
            fov: 60.0_f32.to_radians(),
            ..default()
        }),
        RLCamera,
        VisibleEntities::default(),
        Frustum::default(),
        GlobalTransform::default(),
        Msaa::default(),
    ));
}

// Add this system to run the setup when handles are ready
pub fn check_and_setup_camera(
    mut commands: Commands,
    ai_gym_state: Res<AIGymState<Actions, Observations>>,
    query: Query<Entity, With<RLCamera>>,
) {
    if !ai_gym_state.lock().unwrap().render_image_handles.is_empty() && query.is_empty() {
        setup_rl_camera(commands, ai_gym_state);
    }
}

pub fn handle_pause(
    mut pause_events: EventReader<EventPause>,
    ai_gym_state: Res<AIGymState<Actions, Observations>>,
    cameras: Query<&Camera, With<RLCamera>>,
    images: Res<Assets<Image>>,
) {
    for _ in pause_events.read() {
        let mut ai_gym_state = ai_gym_state.lock().unwrap();

        for camera in cameras.iter() {
            if let RenderTarget::Image(image_handle) = &camera.target {
                if let Some(image) = images.get(image_handle) {
                    let viewport_size = camera
                        .physical_viewport_size()
                        .unwrap_or_else(|| camera.physical_target_size().unwrap());

                    let pixel_data = image.data.clone();

                    let obs = Observations { pixel_data };

                    ai_gym_state.set_env_state(obs);
                    break;
                }
            }
        }
    }
}

pub fn process_actions(
    mut control_events: EventReader<EventControl>,
    mut query: Query<&mut Transform, With<RLCamera>>,
) {
    for control in control_events.read() {
        let unparsed_actions = &control.0;
        for i in 0..unparsed_actions.len() {
            if let Some(unparsed_action) = unparsed_actions[i].clone() {
                let action: Vec<f64> = serde_json::from_str(&unparsed_action).unwrap();

                // Apply the actions to your camera
                for mut transform in query.iter_mut() {
                    transform.translation +=
                        Vec3::new(action[0] as f32, action[1] as f32, action[2] as f32);
                }
            }
        }
    }
}
