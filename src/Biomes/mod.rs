// This file is mostly to be an entry point for the enviornment module
pub mod buildings;
pub mod noise;
pub mod style_biome;
pub mod terrain;

//use crate::biomes::style_biome::BiomeStyle;
use crate::biomes::terrain::setup_terrain;
use crate::AppState;
use bevy::prelude::*;

//use buildings::setup_buildings;

/// EnvironmentPlugin is responsible for setting up the environment for the simulation.
pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        // Add the terrain setup system to run at startup
        app.add_startup_systems(setup_terrain_system);
    }
}

fn setup_terrain_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    setup_terrain(&mut commands, &mut meshes, &mut materials);
}