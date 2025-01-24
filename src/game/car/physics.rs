// physics.rs - Handles car movement and physics systems

use super::car::Car;
use bevy::prelude::*;

// system that handles car movement
pub fn move_car(
    keyboard_input: Res<ButtonInput<KeyCode>>,      // Access to keyboard
    mut car_query: Query<(&mut Car, &mut Transform)>,   // Query to get car components
    time: Res<Time>,                                // Time resource for frame-independent movement
) {
    // Try to get the car entity. if found, continue with movement
    if let Ok((mut car, mut transform)) = car_query.get_single_mut() {
        let delta = time.delta_secs();        // Time since last frame
        let forward = transform.forward();   // get car's forward direction
        let mut rotation = 0.0;               // Inital rotation amount
        let mut is_moving = false;           

        // Acceleration and Deceleration Logic
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            // Accelerate forward
            car.current_speed += car.acceleration * delta;
            car.current_speed = car.current_speed.min(car.max_speed);
            is_moving = true;
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            // Accelerate in reverse
            car.current_speed -= car.acceleration * delta;
            car.current_speed = car.current_speed.max(car.max_reverse_speed);
            is_moving = true;
        } else {
            // Apply deceleration to slow down naturally
            if car.current_speed > 0.0 {
                car.current_speed -= car.deceleration * delta;
                car.current_speed = car.current_speed.max(0.0);
            } else if car.current_speed < 0.0 {
                car.current_speed += car.deceleration * delta;
                car.current_speed = car.current_speed.min(0.0);
            }
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
        let movement = forward * car.current_speed;
        transform.translation += movement * delta;
        transform.rotate_y(rotation)
    }
}