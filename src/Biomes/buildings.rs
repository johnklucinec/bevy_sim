/* /*! This file is to show the use of perlin noise creating basic "buildings" and generating noise values
at a random value to create a simple 3d scene
This is a simple example of how I plan to use perlin noise to create more complex
enviorments in the future */

use bevy::prelude::*;
use crate::biomes::noise::get_scaled_building_height;
use crate::biomes::style_biome::BiomeStyle;
use bevy::math::primitives::Cuboid;



pub fn setup_buildings(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    scale: f64,
    biome_style: BiomeStyle, // Add biome_style parameter to use colors
){
    //loop through a grid of coordinates to place 
    for x in -50..50 {
        for y in -50..50 {
            // Get the height of the building using the noise function
            let building_height = get_scaled_building_height(x as f64, y as f64, scale, 42); // This determines the building's height

            //Use the height from Perlin noise to determine the building size
            //building width and depth can still be constants or generated similarly if needed
            let building_width = 10.0;
            let building_depth = 10.0;

            //spawn the building
        commands.spawn((
            Mesh3d(meshes.add(Mesh::from(Cuboid::new(building_width, building_height as f32, building_depth)))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: biome_style.building_color, // Use color from BiomeStyle
                ..Default::default()
            })),
            Transform {
                translation: Vec3::new(x as f32 * 15.0, building_height as f32 * 0.5, y as f32 * 15.0),
                scale: Vec3::new(1.0, 1.0, 1.0), //default scale; adjust as needed
                ..Default::default()
            },
            Visibility::default(),
        ));
        }
    }
}
 */