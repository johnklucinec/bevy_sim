//Spawning in chunks of terrain to create the enviornment
// around the roads

use crate::game::biome::roadspline::Spline;
use crate::game::terrain::noisewrapper::NoisePerlin;
use crate::game::terrain::TerrainSettings;
use bevy::pbr::MeshMaterial3d;
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};
use bevy::prelude::*;
use noise::NoiseFn;

use super::TerrainMaterial;

#[derive(Component)]
pub struct Chunk {
    pub coord: IVec2,
}

pub fn spawn_chunk(
    cmds: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    _materials: &mut ResMut<Assets<StandardMaterial>>,
    chunk_coord: IVec2,
    road: &Spline,
    perlin: &NoisePerlin,
    set: &TerrainSettings,
    player_tf: &Transform,
    material: Handle<StandardMaterial>,
) {
    let car_x = player_tf.translation.x;
    let car_z = player_tf.translation.z;
    // Creates a grid of 3d vertices to represent a terrain chunk using perlin noise
    // first it will get the world pos, then determines how far apart each vertex is on the grid
    // then it creates a vector to hold 3d pos
    let world = chunk_coord.as_vec2() * set.chunk_size as f32;
    let step = set.chunk_size as f32 / set.verts_per_side as f32;

    //get half road width
    let half_width = set.road_width * 0.5;
    let blend = set.road_blend_distance;
    let flat_h = Spline::HEIGHT;

    let mut verts = Vec::<[f32; 3]>::with_capacity(((set.verts_per_side + 1).pow(2)) as usize);
    for z in 0..=set.verts_per_side {
        for x in 0..=set.verts_per_side {
            let px = world.x + x as f32 * step;
            let pz = world.y + z as f32 * step;

            let rel_x = (px - car_x) * set.freq as f32 + chunk_coord.x as f32 * 1000.0;
            let rel_z = (pz - car_z) * set.freq as f32 + chunk_coord.y as f32 * 1000.0;

            let noise_h = perlin.get([rel_x as f64, rel_z as f64]) as f32 * set.amp;

            // keep the road strip flat
            let dist = road.distance_to(Vec2::new(px, pz));

            let h = if dist <= half_width {
                flat_h
            } else if dist <= half_width + blend {
                let t = (dist - half_width) / blend;
                flat_h * (1.0 - t) + noise_h * t
            } else {
                noise_h
            };

            verts.push([px, h, pz]);
        }
    }

    //build indices for a grid mesh
    let mut indices = Vec::new();
    let verts_per_side = set.verts_per_side + 1;
    for z in 0..set.verts_per_side {
        for x in 0..set.verts_per_side {
            let top_left = z * verts_per_side + x;
            let top_right = top_left + 1;
            let bottom_left = top_left + verts_per_side;
            let bottom_right = bottom_left + 1;

            //first triangle
            indices.push(top_left);
            indices.push(bottom_left);
            indices.push(top_right);
            //second triangle
            indices.push(top_right);
            indices.push(bottom_left);
            indices.push(bottom_right);
        }
    }

    // Spawning in tiles to use textures on the hills
    //repeating for each chunk
    let tile_factor = 4.0;
    let mut uvs = Vec::with_capacity(((set.verts_per_side + 1).pow(2)) as usize);
    for z in 0..=set.verts_per_side {
        for x in 0..=set.verts_per_side {
            let u = (x as f32 / set.verts_per_side as f32) * tile_factor;
            let v = (z as f32 / set.verts_per_side as f32) * tile_factor;
            uvs.push([u, v]);
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, Default::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    let mesh_handle = meshes.add(mesh);

    cmds.spawn((
        Mesh3d(mesh_handle),
        MeshMaterial3d(material),
        Transform::default(),
    ))
    .insert(Chunk { coord: chunk_coord });
}

// 3x3 grid of chunks around around 0,0 to start
pub fn spawn_initial_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    terrain_settings: Res<TerrainSettings>,
    perlin: Res<NoisePerlin>,
    road: Spline,
) {
    let rock_material: Handle<StandardMaterial> =
        asset_server.load("3dmodels/rock_ground/rocks_ground_02_4k.gltf#Material0");

    commands.insert_resource(TerrainMaterial(rock_material.clone()));

    let dummy_tf = Transform::default();
    let range = -1..=1;
    for x in range.clone() {
        for z in range.clone() {
            let chunk_coord = IVec2::new(x, z);

            spawn_chunk(
                &mut commands,
                &mut meshes,
                &mut materials,
                chunk_coord,
                &road,
                &*perlin,
                &terrain_settings,
                &dummy_tf,
                rock_material.clone(),
            );
        }
    }
}
