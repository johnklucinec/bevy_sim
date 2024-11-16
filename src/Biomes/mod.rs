// This file is mostly to be an entry point for the enviornment module
pub mod buildings; //declares the buildings submodulepub mod roads;        
pub mod terrain; //declares the roads submodule
pub mod style; //declares the style submodule(thinking of this one)

// Import setup functions from submodules
use crate::environment::buildings::setup_buildings;  // Import the setup_buildings function
use crate::environment::terrain::setup_terrain;      // Import the setup_terrain function
use crate::environment::style::BiomeStyle;           // Import BiomeStyle from the style module

//Environment setup function to initialize everything
pub fn setup_environment(commands: &mut Commands) {
    //setting up roads
    setup_roads(commands);

    //setting up buildings
    setup_buildings(commands);

    //setting up terrain
    setup_terrain(commands);
}