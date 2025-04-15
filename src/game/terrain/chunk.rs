//Spawning in chunks of terrain to create the enviornment
// around the roads

use bevy::prelude::*;
use noise::Perlin;
use crate::game::terrain::TerrainSettings;
use crate::game::biome::roadspline::Spline;
use crate::game::terrain::noisewrapper::NoisePerlin;
use noise::NoiseFn;
use bevy::pbr::MeshMaterial3d;
use bevy::render::mesh::{Mesh, Indices, PrimitiveTopology};


#[derive(Component)]
pub struct Chunk {
    pub coord: IVec2,
}


pub fn spawn_chunk(
    cmds: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    chunk_coord: IVec2,
    road: &Spline,
    perlin: &NoisePerlin,
    set: &TerrainSettings,
){

    // Creates a grid of 3d vertices to represent a terrain chunk using perlin noise
    // first it will get the world pos, then determines how far apart each vertex is on the grid
    // then it creates a vector to hold 3d pos
    let world = chunk_coord.as_vec2() * set.chunk_size as f32;
    let step = set.chunk_size as f32 / set.verts_per_side as f32;

    let mut verts = Vec::<[f32; 3]>::with_capacity(((set.verts_per_side+1).pow(2)) as usize);
    for z in 0..=set.verts_per_side {
        for x in 0..=set.verts_per_side {
            let px = world.x + x as f32 * step;
            let pz = world.y + z as f32 * step;
            let mut h = perlin.get([px as f64 * set.freq, pz as f64 * set.freq]) as f32 * set.amp;

            // keep the road strip flat
            if road.distance_to(Vec3::new(px, 0.0, pz)) < set.road_clearance {
                h = Spline::HEIGHT;          
            }
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

    
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, Default::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);
    mesh.insert_indices(Indices::U32(indices));

    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.8, 0.3),
        ..Default::default()
    });

    cmds.spawn((
        Mesh3d(mesh_handle),
        MeshMaterial3d(material_handle),
        Transform::default(),
    ))
    .insert(Chunk { coord: chunk_coord });
}


// 3x3 grid of chunks around around 0,0 to start
pub fn spawn_initial_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    terrain_settings: Res<TerrainSettings>,
    perlin: Res<NoisePerlin>,
    road: Res<Spline>,
) {
    
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
            );
        }
    }
}