// This file handles the meshe generation for the ground level of the enviornment

use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

pub mod chunk;
pub mod updatechunk;
mod noisewrapper;

pub use chunk::spawn_initial_chunks;
pub use updatechunk::update_chunks;
use noisewrapper::NoisePerlin;
pub struct TerrainPlugin;


#[derive(Resource)]
pub struct TerrainSettings{
    pub chunk_size: u32,
    pub verts_per_side: u32,
    pub amp: f32,
    pub freq: f64,
    pub road_clearance: f32,
}

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TerrainSettings {
                chunk_size: 64,
                verts_per_side: 64,
                amp: 8.0,
                freq: 0.05,
                road_clearance: 3.0,
            })
            .insert_resource(NoisePerlin(Perlin::new(42)))         
            .add_systems(Startup, spawn_initial_chunks)
            .add_systems(Update, update_chunks);
    }
}