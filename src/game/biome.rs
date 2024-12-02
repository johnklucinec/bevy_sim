
use bevy::prelude::*;
use bevy::render::mesh::Mesh;

//Want to create a biome plugin eventually that is working good

//spawns a simple environment with a flat ground and a road
pub fn setup_terrain(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    //material for the terrain (e.g., grass)
    let terrain_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.5, 0.1), // Greenish for grass
        ..Default::default()
    });
    
    //spawn terrain (ground plane)
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(50.0, 0.1, 50.0)))),
        MeshMaterial3d(terrain_material.clone()),
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0), //y was -0.1
            ..Default::default()
        },
    ));

    
}

