use bevy::{
    pbr::ScreenSpaceAmbientOcclusion,
    prelude::*,
    render::camera::{Exposure, PhysicalCameraParameters},
};

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
        Exposure::from_physical_camera(PhysicalCameraParameters {
            aperture_f_stops: 1.0,
            shutter_speed_s: 1.0 / 250.0,
            sensitivity_iso: 100.0,
            ..default()
        }),
    ));

    commands.spawn((
        PointLight {
            intensity: 4500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 100.0, 4.0),
    ));

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 100.0,
    });
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

pub fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<MoveableCamera>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = camera_query.get_single_mut() {
        let movement_speed = 8.0;
        let delta = time.delta_secs() * movement_speed;

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

        if keyboard_input.pressed(KeyCode::KeyQ) {
            let up_direction = transform.up();
            transform.translation -= up_direction * delta;
        }

        if keyboard_input.pressed(KeyCode::KeyE) {
            let up_direction = transform.up();
            transform.translation += up_direction * delta;
        }
    }
}
