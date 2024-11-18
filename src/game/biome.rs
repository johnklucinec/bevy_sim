use bevy::prelude::*;
use bevy::render::mesh::Mesh;

//Want to create a biome plugin eventually that is working good

//spawns a simple environment with a flat ground and a road
pub fn setup_terrain(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    //material for the road (dark color)
    let road_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.2, 0.2),
        ..Default::default()
    });
    //material for the white strip (white)
    let strip_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 1.0, 1.0),
        ..Default::default()
    });

    // Horizontal road
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(20.0, 1.0, 2.0)))),
        MeshMaterial3d(road_material.clone()),
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(2.0, 0.1, 2.0),
            ..Default::default()
        },
        Visibility::default(),
    ));

    // Vertical road
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(2.0, 1.0, 20.0)))),
        MeshMaterial3d(road_material.clone()),
        Transform {
            translation: Vec3::new(0.0, 0.0, 3.0),
            scale: Vec3::new(2.0, 0.1, 10.0),
            ..Default::default()
        },
        Visibility::default(),
    ));

    // White strip down the middle of the horizontal road
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(20.0, 0.1, 0.5)))),
        MeshMaterial3d(strip_material.clone()),
        Transform {
            translation: Vec3::new(0.0, 0.1, 0.0),
            scale: Vec3::new(2.0, 0.1, 0.5),
            ..Default::default()
        },
        Visibility::default(),
    ));

    // White strip down the middle of the vertical road
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(0.5, 0.1, 100.0)))),
        MeshMaterial3d(strip_material),
        Transform {
            translation: Vec3::new(0.0, 0.05, 0.0),
            scale: Vec3::new(0.5, 0.05, 10.0),
            ..Default::default()
        },
        Visibility::default(),
    ));
}
