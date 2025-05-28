/// Author: Brant Cass (@brantcass)
/* road.rs is a module that creates roads and contains their components creating vertical
and horizontal roads. */
use bevy::prelude::*;
use bevy::render::mesh::Mesh;

use super::rand_objects::spawn_cones_on_road;


#[derive(Component)]
pub struct Road;

#[derive(Clone)]
pub struct Segment {
    pub start: Vec3,
    pub end: Vec3,
}

/*Testing roads for reinforcment AI learning, simple road with railing on the sides.*/
pub fn spawn_single_road(
    commands: &mut Commands,
    asset_server: &AssetServer,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    start: Vec3,
    end: Vec3,
) -> Vec<Segment> {

    let delta = end - start;
    let dx = delta.x;
    let dz = delta.z;
    let distance = delta.length();
    let angle = dz.atan2(dx);

    let road_width = 10.0;
    let road_thickness = 0.1;

    let parent_id = commands
        .spawn((
            Road,
            Mesh3d(meshes.add(Mesh::from(Cuboid::new(
                distance,
                road_thickness,
                road_width,
            )))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.1, 0.1, 0.1),
                metallic: 0.0, // non-metal
                perceptual_roughness: 1.0,
                ..Default::default()
            })),
            Transform {
                translation: Vec3::new(0.0, 0.0, -60.0),
                rotation: Quat::from_rotation_y(angle),
                ..Default::default()
            },
        ))
        .id();

    // Spawn cones randomly
    spawn_cones_on_road(
        commands,
        asset_server,
        parent_id,
        distance,
        road_width,
        road_thickness,
        20,
        15.0,
    );

    //for finish and start line
    let half_x_thickness = 0.2;
    let half_y_thickness = 0.01;
    let half_z_width = road_width;

    //Green start line
    let start_line_mesh = meshes.add(Mesh::from(Cuboid::new(
        half_x_thickness,
        half_y_thickness,
        half_z_width,
    )));

    let start_line_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 1.0, 0.0),
        ..Default::default()
    });

    //Finish line
    let finish_line_mesh = start_line_mesh.clone();
    let finish_line_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.0, 0.0),
        ..Default::default()
    });

    //Center white line
    let center_line_thickness = 0.01;
    let center_line_width = 0.1;

    let center_line_mesh = meshes.add(Mesh::from(Cuboid::new(
        distance,
        center_line_thickness,
        center_line_width,
    )));
    let center_line_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..Default::default()
    });

    //goes from distance x to distance -x
    let total_road_length = distance * 2.0;

    let stop_sign_handle: Handle<Scene> =
        asset_server.load("3dmodels/stop_sign/stop_sign/scene.gltf#Scene0");

    let sign_spacing = 10.0;
    let mut num_signs = (total_road_length / sign_spacing).floor() as i32;

    let left_edge_z = -road_width * 0.5 - 0.3;
    let right_edge_z = road_width * 0.5 + 0.3;

    if num_signs < 1 {
        num_signs = 1;
    }

    //stop sign placement
    commands.entity(parent_id).with_children(|parent| {
        for i in 0..=num_signs {
            let fraction = i as f32 / num_signs as f32;
            let local_x = -distance + fraction * (6.0 * distance);

            //left edge
            parent.spawn((
                SceneRoot(stop_sign_handle.clone()),
                Transform {
                    translation: Vec3::new(local_x, road_thickness + -0.4, left_edge_z),
                    rotation: Quat::from_rotation_y(angle),
                    scale: Vec3::splat(0.5),
                },
                GlobalTransform::default(),
                Visibility::default(),
            ));

            //right edge
            parent.spawn((
                SceneRoot(stop_sign_handle.clone()),
                Transform {
                    translation: Vec3::new(local_x, road_thickness + -0.4, right_edge_z),
                    rotation: Quat::from_rotation_y(angle),
                    scale: Vec3::splat(0.5),
                },
                GlobalTransform::default(),
                Visibility::default(),
            ));
        }

        // Center line
        parent.spawn((
            Mesh3d(center_line_mesh),
            MeshMaterial3d(center_line_material),
            Transform {
                translation: Vec3::new(
                    0.0,
                    road_thickness * 0.5 + center_line_thickness * 0.5,
                    0.0,
                ),
                ..Default::default()
            },
        ));

        // Green line at the "start" (local x = -distance/2)
        parent.spawn((
            Mesh3d(finish_line_mesh.clone()),
            MeshMaterial3d(start_line_material),
            Transform {
                translation: Vec3::new(
                    -distance / 2.0, // Start
                    road_thickness + half_y_thickness,
                    0.0,
                ),
                ..Default::default()
            },
        ));

        //Finish Line
        parent.spawn((
            Mesh3d(finish_line_mesh.clone()),
            MeshMaterial3d(finish_line_material),
            Transform {
                translation: Vec3::new(
                    distance / 2.0, // far end
                    road_thickness + half_y_thickness,
                    0.0,
                ),
                ..Default::default()
            },
        ));
    });

    vec![Segment { start, end }]
}
