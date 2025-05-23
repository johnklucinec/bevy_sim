// Implementation of randomized roads using DFS
// This function spawns a grid of roads using rows, cols, and spacing. Creates a 5x5 10 units apart
// Not in use due to change of directoin in the simulation
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//spawns a grid of roads using rows, cols, and spacing. Creates a 5x5 10 units apart


use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;

#[derive(Component)]
pub struct Road;



#[allow(dead_code)]
pub fn spawn_grid_roads(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,

    rows: usize,
    cols: usize,
    spacing: f32,
) {
    let mut rng = rand::thread_rng();

    //single shared road material
    let road_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.2, 0.2),
        ..Default::default()
    });

    let dash_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..Default::default()
    });

    //Rand roads

    let mut node_positions = vec![Vec2::ZERO; rows * cols];

    for r in 0..rows {
        for c in 0..cols {
            let x = c as f32 * spacing;
            let z = r as f32 * spacing;

            node_positions[r * cols + c] = Vec2::new(x, z);
        }
    }

    let directions = [
        (0isize, 1isize), // Right
        (1, 0),           // Down
        (0, -1),          //left
        (-1, 0),          //up
    ];

    //helper func to convert (r, c) to index
    let index = |r: isize, c: isize| -> Option<usize> {
        if r >= 0 && r < rows as isize && c >= 0 && c < cols as isize {
            Some((r as usize) * cols + (c as usize))
        } else {
            None
        }
    };

    //isize - pointer sized int type and starting at random node
    let start_r = rng.gen_range(0..rows) as isize;
    let start_c = rng.gen_range(0..cols) as isize;

    //dfs stack
    let mut stack = vec![(start_r, start_c)];

    //define directions and visited
    let mut visited = vec![false; cols * rows];

    let mut road_segments: HashSet<(usize, usize)> = HashSet::new();

    //These 2 variables are used to limit the number of connections per node
    let mut node_connections = vec![0; rows * cols];
    let max_connections_per_node = 3;

    while let Some((current_r, current_c)) = stack.pop() {
        let current_idx = match index(current_r, current_c) {
            Some(idx) => idx,
            None => continue,
        };
        if visited[current_idx] {
            continue;
        }

        visited[current_idx] = true;

        //randomize
        let mut dirs = directions;
        dirs.shuffle(&mut rng);

        for &(dr, dc) in dirs.iter() {
            let new_r = current_r + dr;
            let new_c = current_c + dc;

            // Determine allowed directions based on current node's position
            let is_edge = |r: isize, c: isize| -> bool {
                r == 0 || r == (rows as isize - 1) || c == 0 || c == (cols as isize - 1)
            };

            //Prevent roads from spawning outward on edges
            if is_edge(current_r, current_c) {
                //define directions
                let allowed_dirs = if current_r == 0 {
                    //Top edge
                    vec![(1, 0), (0, 1), (0, -1)]
                } else if current_r == (rows as isize - 1) {
                    //bottom edge
                    vec![(-1, 0), (0, 1), (0, -1)]
                } else if current_c == 0 {
                    //left edge
                    vec![(0, 1), (1, 0), (-1, 0)]
                } else if current_c == (cols as isize - 1) {
                    //right edge
                    vec![(0, -1), (1, 0), (-1, 0)]
                } else {
                    directions.to_vec().clone()
                };

                //check if current direction is allowed
                if !allowed_dirs.contains(&(dr, dc)) {
                    continue; // Skip this direction
                }
            }

            if let Some(new_idx) = index(new_r, new_c) {
                if !visited[new_idx] {
                    let segment = if current_idx < new_idx {
                        (current_idx, new_idx)
                    } else {
                        (new_idx, current_idx)
                    };

                    //skips if already connected
                    // not sure adding a ! works here but it spawns roads by themself and roads have incorrect edges.
                    if road_segments.contains(&segment) {
                        continue;
                    }

                    //limit connections per node
                    if node_connections[current_idx] >= max_connections_per_node {
                        continue;
                    }
                    if node_connections[new_idx] >= max_connections_per_node {
                        continue;
                    }

                    //spawning road between (curr_r, curr_c) and (new_r, new_c)
                    let current_pos = node_positions[current_idx];
                    let new_pos = node_positions[new_idx];

                    spawn_road_segment(
                        commands,
                        meshes,
                        &road_material,
                        &dash_material,
                        current_pos,
                        new_pos,
                    );

                    //record road segement
                    road_segments.insert(segment);

                    //counting connections that nodes have
                    node_connections[current_idx] += 1;
                    node_connections[new_idx] += 1;

                    //add new node to stack
                    stack.push((new_r, new_c));
                }
            }
        }
    }

    //Change this to increase or decrease the chance of extra road connections
    let extra_connection_chance = 0.35;

    for r in 0..rows {
        for c in 0..cols {
            let current_idx = r * cols + c;
            let current_pos = node_positions[current_idx];

            for &(dr, dc) in directions.iter() {
                let new_r = r as isize + dr;
                let new_c = c as isize + dc;

                if let Some(new_idx) = index(new_r, new_c) {
                    //road segment as a sorted tuple
                    let segment = if current_idx < new_idx {
                        (current_idx, new_idx)
                    } else {
                        (new_idx, current_idx)
                    };

                    if !road_segments.contains(&segment) && rng.gen_bool(extra_connection_chance) {
                        //Checking node connections limit
                        if node_connections[current_idx] >= max_connections_per_node {
                            continue;
                        }
                        if node_connections[new_idx] >= max_connections_per_node {
                            continue;
                        }

                        let new_pos = node_positions[new_idx];
                        spawn_road_segment(
                            commands,
                            meshes,
                            &road_material,
                            &dash_material,
                            current_pos,
                            new_pos,
                        );
                        road_segments.insert(segment);
                    }
                }
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

    if !(dx == 0.0 || dz == 0.0) {
        println!(
            "Warning atempting to spawn a slanted road segment from ({}, {}) to ({}, {}).",
            start.x, start.y, end.x, end.y
        )
    }

    //midpoint
    let mid_x = (start.x + end.x) / 2.0;
    let mid_z = (start.y + end.y) / 2.0;

    //angle in XZ plane (atan2 takes (y,x) in 2d, but in this y = dz and x = dx)
    let angle = dx.atan2(dz);

    let road_width = 4.0;
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
    let dash_width = 0.15;
    let dash_total = dash_length + dash_space;

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
