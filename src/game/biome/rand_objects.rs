use bevy::prelude::*;
use rand::{thread_rng, Rng};

/// Spawns a specified number of traffic cones randomly along a road segment,
/// ensuring a minimum distance between them.
pub fn spawn_cones_on_road(
    commands: &mut Commands,
    asset_server: &AssetServer,
    road_parent_id: Entity,
    road_distance: f32,
    road_width: f32,    
    road_thickness: f32,
    num_cones: usize,
    min_spacing: f32,
) {
    let cone_handle: Handle<Scene> = asset_server.load("3dmodels/traffic_cone2/scene.gltf#Scene0");
    let mut rng = thread_rng();
    let mut placed_cone_positions: Vec<Vec2> = Vec::with_capacity(num_cones);

    let half_distance = road_distance / 2.0;
    // Keep cones slightly away from the absolute edges
    let spawnable_half_width = road_width / 2.0 - 1.0; 

    const MAX_PLACEMENT_ATTEMPTS: u32 = 100;

    for _ in 0..num_cones {
        let mut attempts = 0;
        loop {
            if attempts >= MAX_PLACEMENT_ATTEMPTS {
                // Avoid infinite loop if space is too crowded
                eprintln!("Warning: Could not place a cone after {} attempts. Skipping.", MAX_PLACEMENT_ATTEMPTS);
                break; 
            }
            attempts += 1;

            // Generate Random Position (relative to road)
            let local_x = rng.gen_range(-half_distance..half_distance);
            let local_z = rng.gen_range(-spawnable_half_width..spawnable_half_width);
            let current_pos_2d = Vec2::new(local_x, local_z);

            //Check Spacing
            let mut too_close = false;
            for &placed_pos in &placed_cone_positions {
                // if cones are too close, try again
                if current_pos_2d.distance(placed_pos) < min_spacing {
                    too_close = true;
                    break;
                }
            }

            // Spawn if spacing is okay
            if !too_close {
                placed_cone_positions.push(current_pos_2d);

                //Small offset above surface
                let local_y = road_thickness / 2.0 + 0.1; 
                
                commands.entity(road_parent_id).with_children(|parent| {
                    parent.spawn((
                        SceneRoot(cone_handle.clone()),
                        Transform {
                            translation: Vec3::new(local_x, local_y, local_z),
                            rotation: Quat::IDENTITY, 
                            scale: Vec3::splat(2.5),
                        },
                        GlobalTransform::default(),
                        Visibility::default(),
                    ));
                });
                break; // Go to the next cone
            }
        }
    }
}