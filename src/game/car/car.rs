// Defines car components and spawning logic

use bevy::{color::palettes::css::GREEN, prelude::*};

#[derive(Component)]
pub struct Car {
    pub speed: f32,      // base movement speed of car
    pub turn_speed: f32, // rotational speed of car
}

// create and spawn car entity into game world
pub fn spawn_car(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
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
    ));
}
