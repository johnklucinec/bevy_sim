use bevy::prelude::*;

///spawns a straight asphalt road with dashed stripes.
pub fn spawn_road(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    road_length: f32,
    road_width: f32,
    turns: usize,
) {
    let road_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.2, 0.2), //asphalt color
        ..Default::default()
    });

    let stripe_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 1.0, 1.0), //stripe color
        ..Default::default()
    });

    let mut current_position = position;
    let mut current_rotation = Quat::IDENTITY;

    for turn in 0..=turns {
        // Spawn the straight road section
        commands.spawn((
            Mesh3d(meshes.add(Mesh::from(Cuboid::new(road_length, 0.1, road_width)))),
            MeshMaterial3d(road_material.clone()),
            Transform {
                translation: current_position,
                rotation: current_rotation,
                ..Default::default()
            },
        ));

        //spawn the dashed lines for the current road segment
        let num_dashes = (road_length / 2.0) as usize;
        let dash_length = road_length / num_dashes as f32;

        if turn % 2 == 0 {
            //horizontal road: Dashed lines along the road's length (X-axis)
            for i in 0..num_dashes {
                let dash_pos = current_position
                    + Vec3::new(
                        -road_length / 2.0 + i as f32 * dash_length + dash_length / 2.0,
                        0.05, // Height of dashed lines
                        0.0,  // No movement along the Z-axis for horizontal lines
                    );

                commands.spawn((
                    Mesh3d(meshes.add(Mesh::from(Cuboid::new(dash_length * 0.8, 0.05, 0.2)))),
                    MeshMaterial3d(stripe_material.clone()),
                    Transform {
                        translation: dash_pos,
                        rotation: current_rotation,
                        ..Default::default()
                    },
                ));
            }
        } else {
            //vertical road: Dashed lines along the road's width (Z-axis)
            for i in 0..num_dashes {
                let dash_pos = current_position
                    + Vec3::new(
                        0.0,  // No movement along the X-axis for vertical lines
                        0.05, // Height of dashed lines
                        -road_width / 2.0 + i as f32 * dash_length + dash_length / 2.0, // Z-axis for vertical lines
                    );

                commands.spawn((
                    Mesh3d(meshes.add(Mesh::from(Cuboid::new(dash_length * 0.8, 0.05, 0.2)))),
                    MeshMaterial3d(stripe_material.clone()),
                    Transform {
                        translation: dash_pos,
                        rotation: current_rotation,
                        ..Default::default()
                    },
                ));
            }
        }

        //update position and rotation for the next road segment (for turns)
        if turn < turns {
            //move the position based on the current rotation
            current_position += current_rotation.mul_vec3(Vec3::new(road_length / 6.0, 0.0, 0.0));
            //rotate 90 degrees for the turn
            current_rotation *= Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
            //90-degree turn
        }
    }
}
