use crate::AppState;
use bevy::prelude::*;

#[derive(Component)]
pub struct MoveableCamera;

// Function that generates the basic 3D scene.
// Just here for to make sure everything runs right.
pub fn setup(
    mut commands: Commands,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        IsDefaultUiCamera,
        MoveableCamera,
    ));

    // Light
    commands.spawn((
        PointLight {
            intensity: 3500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

/// Transition to the game state when the 'G' key is pressed.
pub fn transition_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) && *app_state.get() != AppState::Game {
        app_state_next_state.set(AppState::Game);
        println!("Entered AppState::Game");
    }
}

/// Goes back to the main menu state when the 'M' key is pressed.
pub fn transition_to_main_menu_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) && *app_state.get() != AppState::MainMenu {
        app_state_next_state.set(AppState::MainMenu);
        println!("Entered AppState::MainMenu");
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





