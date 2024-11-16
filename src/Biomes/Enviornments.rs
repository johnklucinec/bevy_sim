// This file will just be used to initalize the enviornment by calling setup
// functions from building.rs and terrain.rs 

use bevy::prelude::*;
use crate::environment::buildings::setup_buildings;
use crate::environment::roads::setup_roads;

pub fn setup_environment(commands: &mut Commands) {
    //setting up roads
    setup_roads(commands);

    //Setting up buildings like houses, etc.
    setup_buildings(commands);
}