/*biome.rs is to create the enviornment around it, I wanted to organize these files
to make it easier to read and alter. Only implimented the grass here. */

use bevy::prelude::*;

pub fn setup_terrain(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    spawn_grass(commands, meshes, materials);
}

pub fn spawn_grass(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    //material for the grass (green color)
    let grass_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.6, 0.1),
        ..Default::default()
    });

    //Flat ground
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(1000.0, 0.1, 1500.0)))),
        MeshMaterial3d(grass_material),
        Transform::from_xyz(0.0, -0.05, 0.0),
    ));
}
