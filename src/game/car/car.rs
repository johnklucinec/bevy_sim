// car.rs - Defines car components and spawning logic

use bevy::{color::palettes::css::GREEN, prelude::*};

#[derive(Component)]
pub struct Car {
    pub current_speed: f32,
    pub turn_speed: f32,
    pub max_speed: f32,
    pub max_reverse_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
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
            current_speed: 5.0,
            turn_speed: 2.5,
            max_speed: 10.0,
            max_reverse_speed: -5.0,
            acceleration: 5.0,
            deceleration: 3.0,
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
        (Vec3::new(-0.5, -0.25, 0.8)),      // Front left
        (Vec3::new(0.5, -0.25, 0.8)),       // Front right
        (Vec3::new(-0.5, -0.25, -0.8)),     // Rear left
        (Vec3::new(0.5, -0.25, -0.8)),      // Rear right
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