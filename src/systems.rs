use crate::AppState;
use bevy::prelude::*;

// #[derive(Component)]
// pub struct Cube;

// Function that generates the basic 3D scene.
// Just here for to make sure everything runs right.
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // // cube
    // commands.spawn((
    //     Cube,
    //     Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    //     MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
    //     Transform::from_xyz(0.0, 0.5, 0.0),
    // ));

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
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        if *app_state.get() != AppState::Game {
            app_state_next_state.set(AppState::Game);
            println!("Entered AppState::Game");
        }
    }
}

/// Goes back to the main menu state when the 'M' key is pressed.
pub fn transition_to_main_menu_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        if *app_state.get() != AppState::MainMenu {
            app_state_next_state.set(AppState::MainMenu);
            println!("Entered AppState::MainMenu");
        }
    }
}

/// Exits the game when the 'ESC' key is pressed.
pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit::Success);
    }
}

// //cube movement
// pub fn move_cube(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut cube_query: Query<&mut Transform, With<Cube>>,
//     time: Res<Time>,
// ) {
//     if let Ok(mut transform) = cube_query.get_single_mut() {
//         let movement_speed = 6.0;
//         let delta = time.delta_secs() * movement_speed;

//         if keyboard_input.pressed(KeyCode::ArrowLeft){
//             transform.translation.x -= delta;
//         }
//         if keyboard_input.pressed(KeyCode::ArrowRight){
//             transform.translation.x += delta;
//         }
//         if keyboard_input.pressed(KeyCode::ArrowUp){
//             transform.translation.z -= delta;
//         }
//         if keyboard_input.pressed(KeyCode::ArrowDown){
//             transform.translation.z += delta;
//         }
//     }
// }
