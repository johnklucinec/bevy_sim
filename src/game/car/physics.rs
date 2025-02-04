// physics.rs - Handles car movement and physics systems

use super::car::{Car, GearMode};
use super::input::CarInput;
use bevy::prelude::*;
use crate::game::road::Road;

// system that handles car movement
pub fn move_car(
    keyboard_input: Res<ButtonInput<KeyCode>>,          // Access to keyboard
    mut car_input: ResMut<CarInput>,                    // Access to car input
    mut car_query: Query<(&mut Car, &mut Transform)>,   // Query to get car components
    time: Res<Time>,                                    // Time resource for frame-independent movement
) {

    // Update CarInput based on keyboard input
    car_input.accelerate = keyboard_input.pressed(KeyCode::ArrowUp);
    car_input.brake = keyboard_input.pressed(KeyCode::ArrowDown);
    car_input.turn_left = keyboard_input.pressed(KeyCode::ArrowLeft);
    car_input.turn_right = keyboard_input.pressed(KeyCode::ArrowRight);
    car_input.toggle_gear = keyboard_input.just_pressed(KeyCode::KeyG);

    // Try to get the car entity. if found, continue with movement
    if let Ok((mut car, mut transform)) = car_query.get_single_mut() {
        let delta = time.delta_secs();      // Time since last frame
        let forward = transform.forward();  // get car's forward direction
        let mut rotation = 0.0;             // Inital rotation amount
        let mut is_moving = false;

        // Gear mode toggle
        if car_input.toggle_gear {
            car.gear_mode = match car.gear_mode {
                GearMode::Forward => GearMode::Reverse,
                GearMode::Reverse => GearMode::Forward,
            };
        }


        // Acceleration and Deceleration Logic
        if !car_input.brake {
            match car.gear_mode {
                GearMode::Forward => {
                    if car_input.accelerate {
                        // Accelerate forward
                        car.current_speed += car.acceleration * delta;
                        car.current_speed = car.current_speed.min(car.max_speed);
                        is_moving = true;
                    } 
                }
                GearMode::Reverse => {
                    if car_input.accelerate {
                        // Accelerate in reverse
                        car.current_speed -= car.acceleration * delta;
                        car.current_speed = car.current_speed.max(car.max_reverse_speed);
                        is_moving = true;
                    }
                }
            }
        }

        if car_input.brake {
            // Progressive braking mechanics
            car.brake_press_duration += delta;
            car.brake_press_duration = car.brake_press_duration.min(car.max_brake_press_duration);
            
            // Calculate dynamic braking force based on press duration
            let brake_intensity = car.brake_press_duration / car.max_brake_press_duration;
            let dynamic_braking_force = car.braking_force * (1.0 + brake_intensity * (car.max_braking_force - car.braking_force));
            
            // Apply braking
            if car.current_speed > 0.0 {
                // Braking when going forward
                car.current_speed -= car.deceleration * dynamic_braking_force * delta;
                car.current_speed = car.current_speed.max(0.0);
            } else if car.current_speed < 0.0 {
                // Braking when going reverse
                car.current_speed += car.deceleration * dynamic_braking_force * delta;
                car.current_speed = car.current_speed.min(0.0);
            }
        } else {
            // Reset brake press duration when Space is not held
            car.brake_press_duration = 0.0;
            
            // Apply friction to naturally slow down the car
            if car.current_speed.abs() > 0.0 {
                let friction_deceleration = car.friction * delta;
                if car.current_speed > 0.0 {
                    car.current_speed -= friction_deceleration;
                    car.current_speed = car.current_speed.max(0.0);
                } else if car.current_speed < 0.0 {
                    car.current_speed += friction_deceleration;
                    car.current_speed = car.current_speed.min(0.0);
                }
            }
        }
        
        // Realistic Turning Mechanics
        let speed_factor = (car.current_speed.abs() / car.max_speed).clamp(0.0, 1.0);
        let turn_sensitivity = 1.0; // Adjust this to fine-tune turning responsiveness

        // Only allow turning when moving and with speed-dependent turn rate
        if is_moving || car.current_speed.abs() > 0.1 {
            if car_input.turn_right {
                // Slower turns at lower speeds 
                rotation -= delta * car.turn_speed * speed_factor * turn_sensitivity;
            }
            if car_input.turn_left {
                // Slower turns at lower speeds
                rotation += delta * car.turn_speed * speed_factor * turn_sensitivity;
            }
        }

        // Apply movement
        let movement = forward * car.current_speed;
        transform.translation += movement * delta;
        transform.rotate_y(rotation);
    }
}

// New system to detect wall collisions and reset car
pub fn reset_car(
    mut car_query: Query<(&mut Car, &mut Transform), With<Car>>,
) {
    if let Ok((mut car, mut transform)) = car_query.get_single_mut() {
        let road_width = 4.0; // Match the road_width from road.rs
        let road_half_width = road_width / 2.0;

        // Check if car is outside the road's width
        if transform.translation.x.abs() > road_half_width {
            // Reset car to original spawn point (0, 0.5, 0)
            transform.translation = Vec3::new(0.0, 0.5, 0.0);
            transform.rotation = Quat::IDENTITY;
            
            // Reset car's speed and other properties
            car.current_speed = 0.0;
            car.gear_mode = GearMode::Forward;
        
        }
    }
}
