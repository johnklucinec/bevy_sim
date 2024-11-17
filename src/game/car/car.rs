// Defines car components and spawning logic

use bevy::prelude::*;

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
        // inital car values
        Car {
            speed: 5.0,      // units per second
            turn_speed: 2.5, // rad per second
        },
        // rectangular box shape
        Mesh3d(meshes.add(Cuboid::new(1.0, 0.5, 2.0))),
        // add green material to car
        MeshMaterial3d(materials.add(Color::srgb(0.0, 0.75, 0.0))),
        // position car slightly above ground in the center of the world
        Transform::from_xyz(0.0, 0.25, 0.0),
    ));
}
