/*! This file is to show the use of perlin noise creating basic "buildings" and generating noise values
at a random value to create a simple 3d scene
This is a simple example of how I plan to use perlin noise to create more complex
enviorments in the future */

use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

pub fn setup_buildings(commands: &mut Commands) {
    for x in 0..5 {
        for y in 0..5 {
            //generate building height using Perlin noise from noise.rs
            let height = get_scaled_building_height(x as f64, y as f64, 10.0); //scale by 10

            let width = 5.0;
            let depth = 5.0;

            commands.spawn_bundle(PbrBundle {
                mesh: bevy::prelude::shape::Box::default(),
                material: StandardMaterial {
                    base_color: Color::rgb(0.7, 0.7, 0.7), // Color for the buildings(gray)
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(x as f32 * 10.0, 0.0, y as f32 * 10.0), //Spaced out on grid
                    scale: Vec3::new(width, height as f32, depth),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}
