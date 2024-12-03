// Handles car movement and physics systems

use super::car::Car;
use bevy::prelude::*;

// system that handles car movement
pub fn move_car(
    keyboard_input: Res<ButtonInput<KeyCode>>, // Access to keyboard
    mut car_query: Query<(&Car, &mut Transform)>, // Query to get car components
    time: Res<Time>,                           // Time resource for frame-independent movement
) {
    // Try to get the car entity. if found, continue with movement
    if let Ok((car, mut transform)) = car_query.get_single_mut() {
        let delta = time.delta_secs(); // Time since last frame
        let forward = transform.forward(); // get car's forward direction
        let mut movement = Vec3::ZERO; // Inital movement vector
        let mut rotation = 0.0; // Inital rotation amount
        let mut is_moving = false;

        // forward/backward movement
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            // move forward
            movement += forward * car.speed;
            is_moving = true;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            // move backward
            movement -= forward * car.speed;
            is_moving = true;
        }

        // rotation (only when moving)
        if is_moving {
            if keyboard_input.pressed(KeyCode::ArrowRight) {
                // rotate clockwise
                rotation -= delta * car.turn_speed;
            }
            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                // rotate counterclockwise
                rotation += delta * car.turn_speed;
            }
        }

        // Apply movement
        transform.translation += movement * delta;
        transform.rotate_y(rotation)
    }
}
