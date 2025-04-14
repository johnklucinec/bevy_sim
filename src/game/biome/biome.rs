/*biome.rs is to create the enviornment around it */

use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use rand::Rng;

pub fn setup_terrain(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    spawn_grass(commands, meshes, materials);
    spawn_trees(commands, meshes, materials);
}

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
