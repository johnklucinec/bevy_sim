/* road.rs is a module that creates roads and contains their components creating vertical
and horizontal roads. */

use bevy::prelude::*;
use bevy::render::mesh::Mesh;

#[derive(Component)]
pub struct Road;

pub fn spawn_roads(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    //material for the road
    let road_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.2, 0.2),
        ..Default::default()
    });

    //Horizontal road
    commands.spawn((
        Road,
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(20.0, 0.1, 5.0)))),
        MeshMaterial3d(road_material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    //Vertical road
    commands.spawn((
        Road,
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(5.0, 0.1, 20.0)))),
        MeshMaterial3d(road_material.clone()),
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));

    //Turn (IN PROGRESS)
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(5.0, 0.1, 5.0)))),
        MeshMaterial3d(road_material.clone()),
        Transform {
            translation: Vec3::new(1.0, 0.0, 20.0),
            rotation: Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
            ..Default::default()
        },
    ));

    //White strip horizontal road
    let strip_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 1.0, 1.0),
        ..Default::default()
    });

    for i in (-9..10).step_by(2) {
        commands.spawn((
            Mesh3d(meshes.add(Mesh::from(Cuboid::new(1.0, 0.05, 0.2)))),
            MeshMaterial3d(strip_material.clone()),
            Transform::from_xyz(i as f32, 0.06, 0.0),
        ));
    }

    //white strip vertical road
    for i in (-9..10).step_by(2) {
        commands.spawn((
            Mesh3d(meshes.add(Mesh::from(Cuboid::new(0.2, 0.05, 1.0)))),
            MeshMaterial3d(strip_material.clone()),
            Transform::from_xyz(0.0, 0.06, i as f32 + 12.0),
        ));
    }
}
