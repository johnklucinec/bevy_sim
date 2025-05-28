//This function checks the current pos, then calculates which chunks should be loaded,
// and it will remove any that fall outside of a certain radius

use crate::game::biome::roadspline::Spline;
use crate::game::car::car::Car;
use crate::game::terrain::chunk::{spawn_chunk, Chunk};
use crate::game::terrain::noisewrapper::NoisePerlin;
use crate::game::terrain::TerrainSettings;
use crate::game::terrain::TerrainMaterial;
use bevy::prelude::*;
use std::collections::HashSet;

pub fn update_chunks(
    mut commands: Commands,
    chunk_query: Query<(Entity, &Chunk)>,
    car_query: Query<&Transform, With<Car>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    terrain_settings: Res<TerrainSettings>,
    terrain_material: Res<TerrainMaterial>,
    perlin: Res<NoisePerlin>,
    road: Res<Spline>,
    
) {


    let player_tf = car_query.single();

    //getting cars position to determine which chunks to load
    let car_transform = match car_query.get_single() {
        Ok(t) => t,
        Err(_) => {
            // no car in world yet skip loading
            return;
        }
    };
    let player_pos = car_transform.translation;

    // get the chunk coordinates based on the player's position
    let chunk_size = terrain_settings.chunk_size as f32;
    let reference_chunk_coord = IVec2::new(
        (player_pos.x / chunk_size).floor() as i32,
        (player_pos.z / chunk_size).floor() as i32,
    );
    // load radius (number of chunks on each side)
    let load_range = 2;
    let mut desired_coords = std::collections::HashSet::new();
    for dx in -load_range..=load_range {
        for dz in -load_range..=load_range {
            desired_coords.insert(reference_chunk_coord + IVec2::new(dx, dz));
        }
    }

    // Despawn any chunk that is not within radius
    for (entity, chunk) in chunk_query.iter() {
        if !desired_coords.contains(&chunk.coord) {
            commands.entity(entity).despawn_recursive();
        }
    }

    let loaded_coords: HashSet<IVec2> = chunk_query.iter().map(|(_, chunk)| chunk.coord).collect();

    //for texutre on hills
    let mat_handle = terrain_material.0.clone();

    for coord in desired_coords {
        
        if !loaded_coords.contains(&coord) {
            
            spawn_chunk(
                &mut commands,
                &mut meshes,
                &mut materials,
                coord,
                &road,
                &*perlin,
                &terrain_settings,
                player_tf,
                mat_handle.clone(),
            );
        }
    }
}
