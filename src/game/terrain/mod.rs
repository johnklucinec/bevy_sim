// This file handles the meshe generation for the ground level of the enviornment

use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

pub struct TerrainPlugin;

#[derive(Resource)]
pub struct TerrainSettings{
    pub chunk_size: u32,
    pub verts_per_side: u32,
    pub amp: f32,
    pub freq: f64,
    pub road_clerance: f32,
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
            .insert_resource(Perlin::new(42))              // global seed
            .add_systems(Startup, spawn_initial_chunks)
            .add_systems(Update, update_chunks);
    }
}