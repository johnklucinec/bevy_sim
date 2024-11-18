use crate::game::ai::pathfinding::a_star;

pub fn move_vehicle_system(
    mut grid: ResMut<Grid>,
    mut query: Query<(&mut GridLocation, &mut Transform)>,
    goal: Res<IVec2>, // Target position
) {
    for (mut location, mut transform) in query.iter_mut() {
        // Get path to the goal
        if let Some(path) = a_star(&grid.entities, IVec2::new(location.x as i32, location.y as i32), *goal) {
            if let Some(next_step) = path.get(1) {
                // Remove from current cell
                grid.entities[location.x][location.y] = None;

                // Move to the next cell
                location.x = next_step.x as usize;
                location.y = next_step.y as usize;
                transform.translation.x = next_step.x as f32;
                transform.translation.z = next_step.y as f32;

                // Place in new cell
                grid.entities[location.x][location.y] = Some(Entity::PLACEHOLDER);
            }
        }
    }
}
