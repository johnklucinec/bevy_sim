use super::components::{
    CarFollowCamera, PythonProcess, SecondaryCamera, SecondaryCameraState, SecondaryWindow,
};
use crate::game::car::car::Car;
use crate::game::SimulationState;
use bevy::prelude::*;
use bevy::render::camera::{RenderTarget, Viewport};
use bevy::window::{WindowRef, WindowResolution};
use std::process::Command;

// Viewport configuration
pub static VIEWPORT_POSITION: UVec2 = UVec2::new(0, 0);
pub static VIEWPORT_SIZE: UVec2 = UVec2::new(500, 500);

pub fn spawn_secondary_camera(mut commands: Commands) {
    let second_window = commands
        .spawn(Window {
            title: "Camera View".into(),
            resolution: WindowResolution::new(VIEWPORT_SIZE.x as f32, VIEWPORT_SIZE.y as f32),
            visible: false,
            ..default()
        })
        .id();

    commands.insert_resource(SecondaryWindow(second_window));

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 5.0, 5.0)).looking_at(Vec3::ZERO, Vec3::Y),
        Camera {
            viewport: Some(Viewport {
                physical_position: VIEWPORT_POSITION,
                physical_size: VIEWPORT_SIZE,
                ..default()
            }),
            order: 1,
            target: RenderTarget::Window(WindowRef::Entity(second_window)),
            is_active: false,
            ..default()
        },
        SecondaryCamera,
        CarFollowCamera,
    ));
}

pub fn update_car_camera(
    car_query: Query<&Transform, With<Car>>,
    mut camera_query: Query<&mut Transform, (With<SecondaryCamera>, Without<Car>)>,
) {
    match (car_query.get_single(), camera_query.get_single_mut()) {
        (Ok(car_transform), Ok(mut camera_transform)) => {
            // Calculate offset based on car's rotation
            let back_offset = car_transform.back() * 10.0; // Multiply by distance behind car
            let up_offset = car_transform.up() * 5.0; // Multiply by height above car

            // Position camera behind and above car
            camera_transform.translation = car_transform.translation + back_offset + up_offset;

            // Look forward from car's position
            let target = car_transform.translation + car_transform.forward() * 10.0;
            camera_transform.look_at(target, Vec3::Y);
        }
        (Err(_), _) => warn!("No car found with Car component"),
        (_, Err(_)) => warn!("No camera found with SecondaryCamera component"),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn toggle_secondary_camera(
    mut camera_query: Query<&mut Camera, With<SecondaryCamera>>,
    input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>,
    simulation_state: Res<State<SimulationState>>,
    camera_state: Res<State<SecondaryCameraState>>,
    mut next_camera_state: ResMut<NextState<SecondaryCameraState>>,
    python_process: ResMut<PythonProcess>,
    second_window: Res<SecondaryWindow>,
) {
    // Checks if the game is running and if TAB was pressed
    if *simulation_state.get() != SimulationState::Running || !input.just_pressed(KeyCode::Tab) {
        return;
    }

    // Ensures camera exists
    let Ok(mut camera) = camera_query.get_single_mut() else {
        return;
    };

    // Ensures camera window exists
    let Ok(mut window) = windows.get_mut(second_window.0) else {
        return;
    };

    match *camera_state.get() {
        SecondaryCameraState::Hidden => {
            window.visible = true;
            camera.is_active = true;
            next_camera_state.set(SecondaryCameraState::Visible);
            load_opencv_script(python_process);
        }
        SecondaryCameraState::Visible => {
            camera.is_active = false;
            window.visible = false;
            next_camera_state.set(SecondaryCameraState::Hidden);
            kill_opencv_script(python_process);
        }
    }
}

pub fn despawn_secondary_camera(
    mut commands: Commands,
    camera_query: Query<Entity, With<SecondaryCamera>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_camera_state: ResMut<NextState<SecondaryCameraState>>,
    python_process: ResMut<PythonProcess>,
    mut windows: Query<&mut Window>,
    second_window: Res<SecondaryWindow>,
) {
    if *simulation_state.get() == SimulationState::Running {
        return;
    }

    // Check if camera window exists
    let Ok(mut window) = windows.get_mut(second_window.0) else {
        return;
    };

    if let Ok(camera_entity) = camera_query.get_single() {
        commands.entity(camera_entity).despawn_recursive();
        next_camera_state.set(SecondaryCameraState::Hidden);
        kill_opencv_script(python_process); // Kills the python script
        window.visible = false;
    }
}

fn load_opencv_script(mut python_process: ResMut<PythonProcess>) {
    python_process.0 = Some(
        Command::new("python")
            .arg("./ai/main.py")
            .spawn()
            .expect("Failed to start Python script"),
    );
}

fn kill_opencv_script(mut python_process: ResMut<PythonProcess>) {
    if let Some(mut child) = python_process.0.take() {
        child.kill().expect("Failed to kill Python script");
    }
}
