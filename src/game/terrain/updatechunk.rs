//This function checks the current pos, then calculates which chunks should be loaded,
// and it will remove any that fall outside of a certain radius

use bevy::prelude::*;
use std::collections::HashSet;
use crate::game::terrain::TerrainSettings;
use crate::game::biome::roadspline::Spline;
use noise::Perlin;
use crate::game::terrain::chunk::{Chunk, spawn_chunk};
use crate::game::terrain::noisewrapper::NoisePerlin;

#[derive(Component)]
pub struct Player;

pub fn update_chunks(
    mut commands: Commands,
    chunk_query: Query<(Entity, &Chunk)>,
    player_query: Query<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    terrain_settings: Res<TerrainSettings>,
    perlin: Res<NoisePerlin>,
    road: Res<Spline>,
) {
    
    let player_transform = player_query.single();
    let player_pos = player_transform.translation;

    //calculate which chunk car is in
    let chunk_size = terrain_settings.chunk_size as f32;
    let player_chunk_coord = IVec2::new(
        (player_pos.x / chunk_size).floor() as i32,
        (player_pos.z / chunk_size).floor() as i32,
    );

    // load radius (number of chunks on each side)
    let load_range = 2; //creates a square of (2*2+1)^2 = 25 chunks around the spawn point
    let mut desired_coords = HashSet::new();
    for dx in -load_range..=load_range {
        for dz in -load_range..=load_range {
            desired_coords.insert(player_chunk_coord + IVec2::new(dx, dz));
        }
    }

    // Despawn any chunk that is not within radius
    for (entity, chunk) in chunk_query.iter() {
        if !desired_coords.contains(&chunk.coord) {
            commands.entity(entity).despawn_recursive();
        }
    }

    let loaded_coords: HashSet<IVec2> =
        chunk_query.iter().map(|(_, chunk)| chunk.coord).collect();

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
            );
        }
    }
}