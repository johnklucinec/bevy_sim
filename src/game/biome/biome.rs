/*biome.rs is to create the enviornment around it, I wanted to organize these files
to make it easier to read and alter. Only implimented the grass here. */

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};
use noise::{NoiseFn, Perlin};
use rand::Rng;

pub fn setup_terrain(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    spawn_grass(commands, meshes, materials);
    //spawn_terrain_with_perlin(commands, meshes, materials);
    spawn_trees(commands, meshes, materials);
}

// pub fn spawn_terrain_with_perlin(
//     commands: &mut Commands,
//     meshes: &mut ResMut<Assets<Mesh>>,
//     materials: &mut ResMut<Assets<StandardMaterial>>,
// ) {
//     let perlin = Perlin::default();

//     let terrain_width = 500;
//     let terrain_depth = 500;

//     let noise_scale = 0.1;
//     let height_amplitude = 10.0;

//     let mut positions = Vec::new();
//     let mut normals = Vec::new();
//     let mut uvs = Vec::new();
//     let mut indices = Vec::new();

//     //generate vertex data from Perlin
//     for z in 0..terrain_depth {
//         for x in 0..terrain_width {
//             let noise_val = perlin.get([x as f64 * noise_scale, z as f64 * noise_scale]);
//             let y = noise_val as f32 * height_amplitude;

//             positions.push([x as f32, y, z as f32]);
//             normals.push([0.0, 1.0, 0.0]);
//             uvs.push([
//                 x as f32 / terrain_width as f32,
//                 z as f32 / terrain_depth as f32,
//             ]);
//         }
//     }

//     // two triangle indices
//     for z in 0..(terrain_depth - 1) {
//         for x in 0..(terrain_width - 1) {
//             let i0 = z * terrain_width + x;
//             let i1 = i0 + 1;
//             let i2 = i0 + terrain_width;
//             let i3 = i2 + 1;

//             indices.extend_from_slice(&[i0, i2, i1, i1, i2, i3]);
//         }
//     }

//     let mut terrain_mesh = Mesh::new(
//         PrimitiveTopology::TriangleList,
//         RenderAssetUsages::default(),
//     );
//     terrain_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
//     terrain_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
//     terrain_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

//     terrain_mesh.insert_indices(Indices::U32(indices));

//     let terrain_material = materials.add(StandardMaterial {
//         base_color: Color::srgb(0.35, 0.25, 0.2),
//         ..Default::default()
//     });

//     commands.spawn((
//         Mesh3d(meshes.add(terrain_mesh)),
//         MeshMaterial3d(terrain_material),
//         Transform::from_xyz(
//             -(terrain_width as f32) / 2.0,
//             0.0,
//             -(terrain_depth as f32) / 2.0,
//         ),
//         GlobalTransform::default(),
//         Visibility::default(),
//     ));
// }

pub fn spawn_grass(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    //material for the grass (green color)
    let grass_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.6, 0.2),
        ..Default::default()
    });

    //Flat ground
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(1000.0, 0.1, 1500.0)))),
        MeshMaterial3d(grass_material),
        Transform::from_xyz(0.0, -0.05, 0.0),
    ));
}

pub fn spawn_trees(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let tree_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.2, 0.1),
        ..Default::default()
    });

    let leaves_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.8, 0.0),
        ..Default::default()
    });

    let mut rng = rand::thread_rng();
    let num_trees = 500; //number of trees to spawn

    for _ in 0..num_trees {
        let x: f32 = rng.gen_range(-500.0..500.0);
        let z: f32 = rng.gen_range(-750.0..750.0);

        //Prevent trees from spawning on roads (assuming road width is 10 units centered at x = 0)
        if x.abs() < 15.0 {
            continue;
        }

        // Tree trunk
        commands.spawn((
            Mesh3d(meshes.add(Mesh::from(Cylinder {
                radius: 0.5,

                ..Default::default()
            }))),
            MeshMaterial3d(tree_material.clone()),
            Transform::from_xyz(x, 0.5, z),
            Visibility::default(),
        ));

        // Spawn tree leaves (as a sphere)
        commands.spawn((
            Mesh3d(meshes.add(Mesh::from(Sphere { radius: 2.0 }))),
            MeshMaterial3d(leaves_material.clone()),
            Transform::from_xyz(x, 3.0, z), //above the trunk
            Visibility::default(),
        ));
    }
}
