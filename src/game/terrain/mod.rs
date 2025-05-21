// This file handles the meshe generation for the ground level of the enviornment

use bevy::prelude::*;
use noise::Perlin;

pub mod chunk;
pub mod noisewrapper;
pub mod updatechunk;

use crate::game::biome::roadspline::Spline;
use crate::game::terrain::noisewrapper::NoisePerlin;
pub use updatechunk::update_chunks;
pub struct TerrainPlugin;

#[derive(Resource)]
pub struct TerrainMaterial(pub Handle<StandardMaterial>);

#[derive(Resource)]
pub struct TerrainSettings {
    pub chunk_size: u32,
    pub verts_per_side: u32,
    pub amp: f32,
    pub freq: f64,
    pub road_width: f32,
    pub road_blend_distance: f32,
}

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TerrainSettings {
            chunk_size: 64,
            verts_per_side: 64,
            amp: 8.0,
            freq: 0.05,
            road_width: 10.0,
            road_blend_distance: 6.0,
        })
        .insert_resource(NoisePerlin(Perlin::new(42)))
        .add_systems(
            Update,
            update_chunks
                .run_if(|world: &World| world.contains_resource::<Spline>())
                .run_if(|world: &World| world.contains_resource::<TerrainMaterial>())
                .run_if(|world: &World| world.contains_resource::<Spline>()),
        );
    }
}
