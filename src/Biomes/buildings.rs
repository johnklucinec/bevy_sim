/*! This file is to show the use of perlin noise creating basic "buildings" and generating noise values
at a random value to create a simple 3d scene
This is a simple example of how I plan to use perlin noise to create more complex
enviorments in the future */

use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

pub fn setup_buildings(commands: &mut Commands) {
    let perlin = Perlin::new(); // create new Perlin noise generator

    for x in 0..5 {
        for y in 0..5 {
            // Perlin noise to vary the height of the buildings to create randomness
            let height = perlin.get([x as f64 * 0.1, y as f64 * 0.1]) * 10.0; //noise output for height

            let width = 5.0;
            let depth = 5.0;

            //using pbr bundel for 3d and the shape of a box as a placeholder for buildings will
            // eventually get into actual models
            commands.spawn_bundle(PbrBundle {
                mesh: bevy::prelude::shape::Box::default(),
                material: StandardMaterial {
                    base_color: Color::rgb(0.7, 0.7, 0.7), //gray color
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(x as f32 * 10.0, 0.0, y as f32 * 10.0), //spaced out on grid
                    scale: Vec3::new(width, height as f32, depth),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}
