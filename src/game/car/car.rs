// Defines car components and spawning logic

use bevy::{color::palettes::css::GREEN, prelude::*};

#[derive(Component)]
pub struct Car {
    pub speed: f32,      // base movement speed of car
    pub turn_speed: f32, // rotational speed of car
}

#[derive(Component)]
pub struct Wheel {
    pub radius: f32,
    pub width: f32
}

// create and spawn car entity into game world
pub fn spawn_car(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn car body
    let car_entity = commands.spawn((
        Car {
            speed: 5.0,      // units per second
            turn_speed: 2.5, // rad per second
        },
        Mesh3d(meshes.add(Cuboid::new(1.0, 0.5, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: GREEN.into(),
            ..Default::default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    )).id();

    // Wheel properties
    let wheel = Wheel {
        radius: 0.25,
        width: 0.2
    };
    let wheel_mesh = meshes.add(Cylinder::new(wheel.width, wheel.radius));
    let wheel_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..Default::default()
    });

    // Wheel positions relative to car body
    let wheel_configs = [
        (Vec3::new(-0.5, -0.25, 0.8)),   // Front left
        (Vec3::new(0.5, -0.25, 0.8)),    // Front right
        (Vec3::new(-0.5, -0.25, -0.8)), // Rear left
        (Vec3::new(0.5, -0.25, -0.8)),  // Rear right
    ];

    // Spawn wheels
    for position in wheel_configs {
        commands.spawn((
            Mesh3d(wheel_mesh.clone()),
            MeshMaterial3d(wheel_material.clone()),
            Transform::from_translation(position)
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),   // rotate wheel 90 degrees
        )).set_parent(car_entity);
    }
}
