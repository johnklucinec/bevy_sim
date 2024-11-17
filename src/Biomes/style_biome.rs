/*I wanted to add this file similar to the other one we have to keep the styles easy to
see and change mostly dealing with the color. I created a color set for the urban enviornment
just to get a start */

use bevy::prelude::*;

// Define a struct to hold biome colors
pub struct BiomeStyle {
    pub road_color: Color,
    pub building_color: Color,
    //pub sky_color: Color,  
}

impl Default for BiomeStyle {
    fn default() -> Self {
        Self::urban_biome() // Default to urban biome style
    }
}

impl BiomeStyle {
    // Urban biome style
    pub fn urban_biome() -> Self {
        Self {
            road_color: Color::srgb(0.3, 0.3, 0.3), // Dark gray for roads
            building_color: Color::srgb(0.8, 0.8, 0.8), // Light gray for buildings
            //sky_color: Color::srgb(0.5, 0.7, 1.0), // Light blue for sky
        }
    }
}
