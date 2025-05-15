use super::components::*;
use crate::game::{
    car::car::Car,
    python::{
        components::PythonComms,
        systems::{setup_io_threads, spawn_python_child},
    },
    SimulationState,
};
use bevy::{prelude::*, render::camera::*, window::*};

// Viewport configuration
pub static VIEWPORT_POSITION: UVec2 = UVec2::new(0, 0);
pub static VIEWPORT_SIZE: UVec2 = UVec2::new(500, 500);

pub fn spawn_secondary_camera(mut commands: Commands) {
    let second_window = commands
        .spawn(Window {
            title: "Camera View".into(),
            resolution: WindowResolution::new(VIEWPORT_SIZE.x as f32, VIEWPORT_SIZE.y as f32),
            visible: false,
            enabled_buttons: EnabledButtons {
                minimize: false,
                maximize: false,
                close: false,
            },
            resizable: false,
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
        Exposure::from_physical_camera(PhysicalCameraParameters {
            aperture_f_stops: 1.0,
            shutter_speed_s: 1.0 / 250.0,
            sensitivity_iso: 100.0,
            ..default()
        }),
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
            let back_offset = car_transform.back() * 3.0; // Multiply by distance behind car
            let up_offset = car_transform.up() * 2.0; // Multiply by height above car

            // Position camera behind and above car
            camera_transform.translation = car_transform.translation + back_offset + up_offset;

            // Look forward from car's position
            let target = car_transform.translation + car_transform.forward() * 30.0;
            camera_transform.look_at(target, Vec3::Y);
        }
        (Err(_), _) => warn!("No car found with Car component"),
        (_, Err(_)) => warn!("No camera found with SecondaryCamera component"),
    }
}

pub fn toggle_secondary_camera(
    mut camera_query: Query<&mut Camera, With<SecondaryCamera>>,
    simulation_state: Res<State<SimulationState>>,
    camera_state: Res<State<SecondaryCameraState>>,
    mut next_camera_state: ResMut<NextState<SecondaryCameraState>>,
    input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>,
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
            // TODO: Stop the car from driving
        }
        SecondaryCameraState::Visible => {
            camera.is_active = false;
            window.visible = false;
            next_camera_state.set(SecondaryCameraState::Hidden);
        }
    }
}

pub fn despawn_secondary_camera(
    mut commands: Commands,
    camera_query: Query<Entity, With<SecondaryCamera>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_camera_state: ResMut<NextState<SecondaryCameraState>>,
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
        next_camera_state.set(SecondaryCameraState::Hidden);
        commands.entity(camera_entity).despawn_recursive();
        window.visible = false;
    }
}

pub fn spawn_python_process(mut commands: Commands) {
    let (child, stdin, stdout) = spawn_python_child();
    let (tx, rx) = crossbeam_channel::bounded(1000);

    setup_io_threads(tx.clone(), stdout);

    commands.insert_resource(PythonComms {
        child,
        stdin,
        tx,
        rx,
    });
}

pub fn kill_python_process(mut commands: Commands, comms: Option<ResMut<PythonComms>>) {
    if let Some(mut comms) = comms {
        comms.child.kill().unwrap();
        commands.remove_resource::<PythonComms>();
    }
}

// Add cleanup system
pub fn cleanup_python_comms(mut commands: Commands, python_comms: Option<Res<PythonComms>>) {
    if python_comms.is_some() {
        commands.remove_resource::<PythonComms>();
    }
}

#[allow(clippy::too_many_arguments)]
pub fn cleanup_system(
    mut commands: Commands,
    exit_events: EventReader<AppExit>,
    camera_query: Query<Entity, With<SecondaryCamera>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_camera_state: ResMut<NextState<SecondaryCameraState>>,
    mut windows: Query<&mut Window>,
    second_window: Res<SecondaryWindow>,
    python_comms: Option<ResMut<PythonComms>>,
) {
    if !exit_events.is_empty() {
        // Kill Python process
        if let Some(mut comms) = python_comms {
            comms.child.kill().unwrap();
            commands.remove_resource::<PythonComms>();
        }

        // Cleanup Python comms
        commands.remove_resource::<PythonComms>();

        // Despawn secondary camera
        if *simulation_state.get() != SimulationState::Running {
            // Check if camera window exists
            let Ok(mut window) = windows.get_mut(second_window.0) else {
                return;
            };

            if let Ok(camera_entity) = camera_query.get_single() {
                next_camera_state.set(SecondaryCameraState::Hidden);
                commands.entity(camera_entity).despawn_recursive();
                window.visible = false;
            }
        }
    }
}
