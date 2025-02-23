use super::{DungeonMap, DungeonPosition, TileType};
use bevy::utils::hashbrown::HashMap;
use std::collections::BinaryHeap;

pub(super) struct AStarPathFinder;

impl AStarPathFinder {
    pub fn find(
        start: DungeonPosition,
        goal: DungeonPosition,
        map: &DungeonMap,
    ) -> PathFindingResult {
        let mut open_set = BinaryHeap::new();
        open_set.push(Node::new(start, 0));

        let mut came_from = HashMap::new();

        let mut node_costs = HashMap::new();
        node_costs.insert(start, 0);

        let mut closest_pos = start;
        let mut closest_distance = manhattan_distance(start, goal);

        while let Some(current) = open_set.pop() {
            if current.pos == goal {
                let mut path_len = 1;
                let mut current = goal;
                while let Some(&prev) = came_from.get(&current) {
                    path_len += 1;
                    current = prev;
                }

                return PathFindingResult::PathLength(path_len);
            }

            [
                DungeonPosition::new(current.pos.x + 1, current.pos.y),
                DungeonPosition::new(current.pos.x - 1, current.pos.y),
                DungeonPosition::new(current.pos.x, current.pos.y + 1),
                DungeonPosition::new(current.pos.x, current.pos.y - 1),
            ]
            .into_iter()
            .filter(|n| map.is_valid_position(&n) && map.get_tile_type(&n) != TileType::Wall)
            .for_each(|neighbor| {
                let tentative_g_score = node_costs.get(&current.pos).unwrap_or(&usize::MAX) + 1;

                let current_distance = manhattan_distance(current.pos, goal);
                if current_distance < closest_distance {
                    closest_distance = current_distance;
                    closest_pos = current.pos;
                }

                if tentative_g_score < *node_costs.get(&neighbor).unwrap_or(&usize::MAX) {
                    came_from.insert(neighbor, current.pos);
                    node_costs.insert(neighbor, tentative_g_score);
                    open_set.push(Node::new(
                        neighbor,
                        tentative_g_score + manhattan_distance(neighbor, goal),
                    ));
                }
            });
        }

        PathFindingResult::ClosestPos(closest_pos)
    }
}

pub(super) enum PathFindingResult {
    ClosestPos(DungeonPosition),
    PathLength(usize),
}

#[derive(Eq, PartialEq)]
struct Node {
    pub cost: usize,
    pub pos: DungeonPosition,
}

impl Node {
    fn new(pos: DungeonPosition, cost: usize) -> Self {
        Self { cost, pos }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn manhattan_distance(a: DungeonPosition, b: DungeonPosition) -> usize {
    (a.x - b.x).abs() as usize + (a.y - b.y).abs() as usize
}
