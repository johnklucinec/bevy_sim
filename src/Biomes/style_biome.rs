/*I wanted to add this file similar to the other one we have to keep the styles easy to
see and change mostly dealing with the color. I created a color set for the urban enviornment
just to get a start */

use bevy::prelude::*;

//define a struct to hold biome colors
#[derive(Default)]
pub struct BiomeStyle {
    pub road_color: Color,
    pub building_color: Color,
    pub sky_color: Color,
}

impl BiomeStyle {
    //create a method to apply an urban biome color scheme
    pub fn urban_biome() -> Self {
        Self {
            road_color: Color::rgb(0.3, 0.3, 0.3), //Dark gray for roads
            building_color: Color::rgb(0.8, 0.8, 0.8), //Light gray for buildings
            //sky_color: Color::rgb(0.5, 0.7, 1.0), //Light blue for sky
        }
    }
}