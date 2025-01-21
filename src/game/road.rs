/* road.rs is a module that creates roads and contains their components creating vertical
and horizontal roads. */

use bevy::prelude::*;
use bevy::render::mesh::Mesh;

#[derive(Component)]
pub struct Road;

//spawns a grid of roads using rows, cols, and spacing. Creates a 5x5 10 units apart
pub fn spawn_grid_roads(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,

    rows: usize,
    cols: usize,
    spacing: f32,
) {
    //single shared road material
    let road_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.2, 0.2),
        ..Default::default()
    });

    let dash_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..Default::default()
    });

    for r in 0..rows {
        for c in 0..cols {
            //curr position of node x and z horizontal plane
            let x = c as f32 * spacing;
            let z = r as f32 * spacing;

            //connecting horizontally
            if c + 1 < cols {
                let x2 = (c + 1) as f32 * spacing;
                let z2 = z;

                spawn_road_segment(
                    commands,
                    meshes,
                    &road_material,
                    &dash_material,
                    Vec2::new(x, z),
                    Vec2::new(x2, z2),
                );
            }
            if r + 1 < rows {
                let x2 = x;
                let z2 = (r + 1) as f32 * spacing;

                spawn_road_segment(
                    commands,
                    meshes,
                    &road_material,
                    &dash_material,
                    Vec2::new(x, z),
                    Vec2::new(x2, z2),
                );
            }
        }
    }
}

fn spawn_road_segment(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: &Handle<StandardMaterial>,
    dash_material: &Handle<StandardMaterial>,
    start: Vec2,
    end: Vec2,
) {
    let dx = end.x - start.x;
    let dz = end.y - start.y;
    let length = (dx * dx + dz * dz).sqrt();

    //midpoint
    let mid_x = (start.x + end.x) / 2.0;
    let mid_z = (start.y + end.y) / 2.0;

    //angle in XZ plane (atan2 takes (y,x) in 2d, but in this y = dz and x = dx)
    let angle = dx.atan2(dz);

    let road_width = 3.0;
    let road_thickness = 0.1;

    commands.spawn((
        //tag
        Road,
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(length, road_thickness, road_width)))),
        MeshMaterial3d(material.clone()),
        Transform {
            translation: Vec3::new(mid_x, 0.0, mid_z),
            //rotate around y axis -> -angle so cuboids x axis matches dx
            rotation: Quat::from_rotation_y(angle),
            ..Default::default()
        },
    ));

    //dashed white lines

    let dash_length = 1.0;
    let dash_space = 1.0;
    let dash_thickness = 0.01;
    let dash_width = 0.1;
    let dash_total = dash_length + dash_space;

    //maybe change .floor and take it off
    let dash_count = (length / dash_total) as usize;

    for i in 0..dash_count {
        //figuring out offset
        //line_offset is so the intersections line up
        let line_offset = 0.5;
        // place the center of the dash at offset x
        let offset_x = -length / 2.0 + dash_length / 2.0 + line_offset + (i as f32 * dash_total);
        let line_y = 0.05 + 0.001;
        let local_dash_pos = Vec3::new(offset_x, line_y, 0.0);

        //rotate around y by angle
        let world_offset = Quat::from_rotation_y(angle) * local_dash_pos;
        let world_position = Vec3::new(mid_x, 0.0, mid_z) + world_offset;

        commands.spawn((
            Mesh3d(meshes.add(Mesh::from(Cuboid::new(
                dash_length,
                dash_thickness,
                dash_width,
            )))),
            MeshMaterial3d(dash_material.clone()),
            Transform {
                translation: world_position,
                rotation: Quat::from_rotation_y(angle),
                ..Default::default()
            },
        ));
    }
}
