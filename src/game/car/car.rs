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
    pub width: f32,
    pub is_front: bool,  // whether this is a front wheel (for steering)
    pub rotation: f32,   // current rotation of the wheel
    pub side: i8,        // -1 for left, 1 for right
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

    // Wheel dimensions
    let wheel_radius = 0.25;
    let wheel_width = 0.2;
    let wheel_mesh = meshes.add(Cylinder::new(wheel_width, wheel_radius));
    let wheel_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..Default::default()
    });

    // Wheel positions relative to car body
    let wheel_configs = [
        (Vec3::new(-0.5, -0.25, 0.8), true, -1),   // Front left
        (Vec3::new(0.5, -0.25, 0.8), true, 1),    // Front right
        (Vec3::new(-0.5, -0.25, -0.8), false, -1), // Rear left
        (Vec3::new(0.5, -0.25, -0.8), false, 1),  // Rear right
    ];

    // Spawn wheels
    for (position, is_front, side) in wheel_configs {
        commands.spawn((
            Wheel {
                radius: wheel_radius,
                width: wheel_width,
                is_front,
                rotation: 0.0,
                side,
            },
            Mesh3d(wheel_mesh.clone()),
            MeshMaterial3d(wheel_material.clone()),
            Transform::from_translation(position)
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
        )).set_parent(car_entity);
    }
}
