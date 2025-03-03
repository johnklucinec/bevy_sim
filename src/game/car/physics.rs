// physics.rs - Handles car movement and physics systems

use super::car::{Car, GearMode};
use super::input::CarInput;
use bevy::prelude::*;
//use crate::game::road::Road;

// system that handles car movement
pub fn move_car(
    keyboard_input: Res<ButtonInput<KeyCode>>, // Access to keyboard
    mut car_input: ResMut<CarInput>,           // Access to car input
    mut car_query: Query<(&mut Car, &mut Transform)>, // Query to get car components
    time: Res<Time>,                           // Time resource for frame-independent movement
) {
    // Update CarInput based on keyboard input
    car_input.accelerate = keyboard_input.pressed(KeyCode::ArrowUp);
    car_input.brake = keyboard_input.pressed(KeyCode::ArrowDown);
    car_input.turn_left = keyboard_input.pressed(KeyCode::ArrowLeft);
    car_input.turn_right = keyboard_input.pressed(KeyCode::ArrowRight);
    car_input.toggle_gear = keyboard_input.just_pressed(KeyCode::KeyG);


    // Parse any text commands
    car_input.parse_text_command();

    // Try to get the car entity. if found, continue with movement
    if let Ok((mut car, mut transform)) = car_query.get_single_mut() {
        let delta = time.delta_secs(); // Time since last frame
        let forward = transform.forward(); // get car's forward direction

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
                    }
                }
                GearMode::Reverse => {
                    if car_input.accelerate {
                        // Accelerate in reverse
                        car.current_speed -= car.acceleration * delta;
                        car.current_speed = car.current_speed.max(car.max_reverse_speed);
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
            let dynamic_braking_force = car.braking_force
                * (1.0 + brake_intensity * (car.max_braking_force - car.braking_force));

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

        // Steering angle control - UPDATED
        if car_input.turn_right {
            car.steering_angle -= car.steering_speed * delta;
            car.steering_angle = car.steering_angle.max(-car.max_steering_angle);
        } else if car_input.turn_left {
            car.steering_angle += car.steering_speed * delta;
            car.steering_angle = car.steering_angle.min(car.max_steering_angle);
        } else {
            // Gradually return steering to center when not turning
            if car.steering_angle > 0.0 {
                car.steering_angle -= car.steering_speed * 0.5 * delta;
                car.steering_angle = car.steering_angle.max(0.0);
            } else if car.steering_angle < 0.0 {
                car.steering_angle += car.steering_speed * 0.5 * delta;
                car.steering_angle = car.steering_angle.min(0.0);
            }
        }

        // Calculate turning radius based on steering angle - NEW
        let turning_radius = if car.steering_angle.abs() > 0.001 {
            car.wheelbase / car.steering_angle.abs().tan()
        } else {
            // If steering angle is close to zero, use a very large radius
            f32::MAX
        };

        // Apply movement based on Ackermann steering model - UPDATED
        if car.current_speed.abs() > 0.1 {
            let speed = car.current_speed * delta;
            
            if car.steering_angle.abs() > 0.001 {
                // Angular velocity is speed divided by turning radius
                let angular_velocity = speed / turning_radius;
                
                // Rotation direction based on steering angle
                let rotation_direction = car.steering_angle.signum();
                
                // Apply rotation, reverse direction when going backwards
                let rotation_amount = angular_velocity * rotation_direction * car.current_speed.signum();
                transform.rotate_y(rotation_amount);
            }
            
            // Move along the current forward vector
            transform.translation += forward * speed;
        }
    }
}

// New system to detect wall collisions and reset car
pub fn reset_car(mut car_query: Query<(&mut Car, &mut Transform), With<Car>>) {
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

// New system to update wheel rotations based on steering angle
pub fn update_wheel_rotations(
    car_query: Query<(&Car, &Children)>,
    mut wheel_transforms: Query<&mut Transform>,
) {
    if let Ok((car, children)) = car_query.get_single() {
        // Iterate through all car's children
        for &child in children.iter() {
            if let Ok(mut wheel_transform) = wheel_transforms.get_mut(child) {
                // Determine if this is a front wheel based on z position
                // Front wheels have positive z value in our coordinate system
                let is_front_wheel = wheel_transform.translation.z > 0.0;
                
                if is_front_wheel {
                    // Apply steering angle to front wheels
                    wheel_transform.rotation = 
                        Quat::from_rotation_z(std::f32::consts::FRAC_PI_2) * 
                        Quat::from_rotation_y(car.steering_angle);
                } else {
                    // Keep rear wheels oriented straight ahead
                    wheel_transform.rotation = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);
                }
            }
        }
    }
}
