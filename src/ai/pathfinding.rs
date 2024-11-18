use std::collections::{BinaryHeap, HashMap};
use bevy::math::IVec2;

// A* Node
#[derive(PartialEq, Eq)]
struct Node {
    position: IVec2,
    cost: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost) // Min-heap
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// A* Pathfinding Algorithm
pub fn a_star(
    grid: &[[Option<Entity>; 10]; 10],
    start: IVec2,
    goal: IVec2,
) -> Option<Vec<IVec2>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();

    open_set.push(Node {
        position: start,
        cost: 0,
    });
    g_score.insert(start, 0);

    while let Some(current) = open_set.pop() {
        if current.position == goal {
            let mut path = vec![goal];
            let mut current = goal;
            while let Some(&prev) = came_from.get(&current) {
                path.push(prev);
                current = prev;
            }
            path.reverse();
            return Some(path);
        }

        for offset in &[
            IVec2::new(1, 0),
            IVec2::new(-1, 0),
            IVec2::new(0, 1),
            IVec2::new(0, -1),
        ] {
            let neighbor = current.position + *offset;

            if neighbor.x < 0
                || neighbor.y < 0
                || neighbor.x >= 10
                || neighbor.y >= 10
                || grid[neighbor.x as usize][neighbor.y as usize].is_some()
            {
                continue;
            }

            let tentative_g_score = g_score[&current.position] + 1;
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&i32::MAX) {
                came_from.insert(neighbor, current.position);
                g_score.insert(neighbor, tentative_g_score);

                open_set.push(Node {
                    position: neighbor,
                    cost: tentative_g_score + (goal - neighbor).abs().x + (goal - neighbor).abs().y,
                });
            }
        }
    }
    None
}
