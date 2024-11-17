/* The terrain file for right now will create a basic road, I was aiming for a simple neighborhood
just for right now. Basically using boxes to form it

The code is using boxes meshes for everything right now to create a intersection as well as the
white strip down the middle
*/

use crate::biomes::style_biome::BiomeStyle;
use bevy::math::primitives::Cuboid;
use bevy::prelude::*;
use bevy::prelude::{Assets, Color, Commands, Mesh, StandardMaterial, Transform};
//use crate::biomes::setup_buildings;

pub fn setup_terrain(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // Create BiomeStyle instance to get color schemes
    let biome_style = BiomeStyle::urban_biome();

    // Create material handles for the road and white strip
    let road_material_handle = materials.add(StandardMaterial {
        base_color: biome_style.road_color, // Dark gray for the road
        ..Default::default()
    });

    let strip_material_handle = materials.add(StandardMaterial {
        base_color: Color::WHITE, // White for the strip
        ..Default::default()
    });

    // Horizontal road
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(100.0, 1.0, 5.0)))),
        MeshMaterial3d(road_material_handle.clone()),
        Transform {
            translation: Vec3::new(0.0, 0.0, 15.0),
            scale: Vec3::new(50.0, 0.1, 5.0),
            ..Default::default()
        },
        Visibility::default(),
    ));

    // Vertical road
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(5.0, 1.0, 100.0)))),
        MeshMaterial3d(road_material_handle.clone()),
        Transform {
            translation: Vec3::new(15.0, 0.0, 0.0),
            scale: Vec3::new(5.0, 0.1, 50.0),
            ..Default::default()
        },
        Visibility::default(),
    ));

    // White strip down the middle of the horizontal road
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(100.0, 0.1, 0.5)))),
        MeshMaterial3d(strip_material_handle.clone()),
        Transform {
            translation: Vec3::new(0.0, 0.1, 15.0),
            scale: Vec3::new(50.0, 0.1, 0.5),
            ..Default::default()
        },
        Visibility::default(),
    ));

    // White strip down the middle of the vertical road
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(0.5, 0.1, 100.0)))),
        MeshMaterial3d(strip_material_handle),
        Transform {
            translation: Vec3::new(15.0, 0.05, 0.0),
            scale: Vec3::new(0.5, 0.05, 50.0),
            ..Default::default()
        },
        Visibility::default(),
    ));
}
