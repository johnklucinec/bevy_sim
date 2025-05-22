/// Author: John Klucinec (@johnklucinec)
use bevy::{
    prelude::*,
    render::camera::{Exposure, PhysicalCameraParameters},
};
use std::f32::consts::FRAC_PI_4;

use crate::{game::car::car::Car, CameraState};

#[derive(Component)]
pub struct MoveableCamera;

// Function that generates the basic 3D scene.
// Just here for to make sure everything runs right.
pub fn setup(mut commands: Commands) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        IsDefaultUiCamera,
        MoveableCamera,
        Projection::Perspective(PerspectiveProjection {
            fov: std::f32::consts::FRAC_PI_4,
            near: 0.1,     // how close can the camera see
            far: 10_000.0, // how far before things disappear
            aspect_ratio: 16.0 / 9.0,
        }),
        Exposure::from_physical_camera(PhysicalCameraParameters {
            aperture_f_stops: 1.0,
            shutter_speed_s: 1.0 / 250.0,
            sensitivity_iso: 100.0,
            ..default()
        }),
    ));

    // commands.spawn((
    //     PointLight {
    //         intensity: 4000.0,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     Transform::from_xyz(4.0, 100.0, 4.0),
    // ));

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 100.0,
    });

    commands.spawn((
        DirectionalLight {
            //full sun
            color: Color::srgb(1.0, 0.98, 0.9), // Changed to a warmer daylight color
            illuminance: 1000.0,
            //might have to turn off shadows for preformance
            shadows_enabled: true,
            //tweak these if your shadows are clipping
            shadow_depth_bias: 0.1,
            shadow_normal_bias: 2.0,
            ..default()
        },
        Transform {
            translation: Vec3::new(4.0, 100.0, 4.0),
            rotation: Quat::from_rotation_x(-FRAC_PI_4),
            ..Default::default()
        },
        GlobalTransform::default(),
    ));
}

pub fn update_car_camera(
    car_query: Query<&Transform, With<Car>>,
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, (With<MoveableCamera>, Without<Car>)>,
) {
    match (car_query.get_single(), camera_query.get_single_mut()) {
        (Ok(car_transform), Ok(mut camera_transform)) => {
            let detla_time = time.delta_secs();
            // Calculate offset based on car's rotation
            let back_offset = car_transform.back() * 10.0; // Multiply by distance behind car
            let up_offset = car_transform.up() * 6.0; // Multiply by height above car

            // Position camera behind and above car
            let target_position = car_transform.translation + back_offset + up_offset;
            let smoothness = 5.0;

            // Smoothly move camera towards target position
            camera_transform.translation = camera_transform
                .translation
                .lerp(target_position, detla_time * smoothness);

            // Look at the car
            camera_transform.look_at(car_transform.translation, Vec3::Y);
        }
        (Err(_), _) => warn!("No car found with Car component"),
        (_, Err(_)) => warn!("No camera found with SecondaryCamera component"),
    }
}

/// Exits the game when the 'ALT + F4' key is pressed.
pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    let alt_pressed = keyboard_input.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]);

    if alt_pressed && keyboard_input.just_pressed(KeyCode::F4) {
        app_exit_event_writer.send(AppExit::Success);
    }
}

pub fn change_camera_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_state: ResMut<NextState<CameraState>>,
) {
    let car_cam_pressed = keyboard_input.just_pressed(KeyCode::Digit1);
    let free_cam_pressed = keyboard_input.just_pressed(KeyCode::Digit2);

    if car_cam_pressed {
        camera_state.set(CameraState::CarCam);
    } else if free_cam_pressed {
        camera_state.set(CameraState::FreeCam);
    }
}

pub fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<MoveableCamera>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = camera_query.get_single_mut() {
        let movement_speed = 8.0;
        let delta = time.delta_secs() * movement_speed;

        // Handle WASD (world axes)
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.translation.x -= delta;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.translation.x += delta;
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.translation.z -= delta;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            transform.translation.z += delta;
        }

        // Handle QE (local up/down)
        if keyboard_input.pressed(KeyCode::KeyQ) || keyboard_input.pressed(KeyCode::KeyE) {
            let up_direction = transform.up();
            if keyboard_input.pressed(KeyCode::KeyQ) {
                transform.translation -= up_direction * delta;
            }
            if keyboard_input.pressed(KeyCode::KeyE) {
                transform.translation += up_direction * delta;
            }
        }
    }
}
