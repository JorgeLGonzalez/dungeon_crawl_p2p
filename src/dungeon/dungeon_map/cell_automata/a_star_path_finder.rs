use super::{AStarNode, DungeonMap, DungeonPosition, TileType};
use bevy::utils::hashbrown::HashMap;
use std::collections::BinaryHeap;

pub(super) struct AStarPathFinder {
    came_from: HashMap<DungeonPosition, DungeonPosition>,
    closest_distance: usize,
    closest_pos: DungeonPosition,
    goal: DungeonPosition,
    node_costs: HashMap<DungeonPosition, usize>,
    open_set: BinaryHeap<AStarNode>,
}

impl AStarPathFinder {
    pub fn find(
        start: DungeonPosition,
        goal: DungeonPosition,
        map: &DungeonMap,
    ) -> PathFindingResult {
        /// return Option and form actual result in this method
        Self::new(goal, start).find_path(map)
    }

    fn new(goal: DungeonPosition, start: DungeonPosition) -> Self {
        let mut open_set = BinaryHeap::new();
        open_set.push(AStarNode::new(start, 0));

        let mut node_costs = HashMap::new();
        node_costs.insert(start, 0);

        let closest_pos = start;
        let closest_distance = start.manhattan_distance(goal);

        Self {
            came_from: HashMap::new(),
            closest_distance,
            closest_pos,
            goal,
            node_costs,
            open_set,
        }
    }

    fn calculate_path_length(&self) -> PathFindingResult {
        let mut path_len = 1;
        let mut current = self.goal;

        while let Some(&prev) = self.came_from.get(&current) {
            path_len += 1;
            current = prev;
        }

        PathFindingResult::PathLength(path_len)
    }

    fn find_path(&mut self, map: &DungeonMap) -> PathFindingResult {
        while let Some(current) = self.open_set.pop() {
            if current.pos == self.goal {
                return self.calculate_path_length();
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
                let tentative_g_score =
                    self.node_costs.get(&current.pos).unwrap_or(&usize::MAX) + 1;

                let current_distance = current.pos.manhattan_distance(self.goal);
                if current_distance < self.closest_distance {
                    self.closest_distance = current_distance;
                    self.closest_pos = current.pos;
                }

                if tentative_g_score < *self.node_costs.get(&neighbor).unwrap_or(&usize::MAX) {
                    self.came_from.insert(neighbor, current.pos);
                    self.node_costs.insert(neighbor, tentative_g_score);
                    self.open_set.push(AStarNode::new(
                        neighbor,
                        tentative_g_score + neighbor.manhattan_distance(self.goal),
                    ));
                }
            });
        }

        PathFindingResult::ClosestPos(self.closest_pos)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(super) enum PathFindingResult {
    /// No path found, but closest position to goal is returned.
    ClosestPos(DungeonPosition),
    /// Path found, length is returned.
    PathLength(usize),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::config::{X_MIN, Y_MAX, Y_MIN};

    #[test]
    fn find_no_floors() {
        let fixture = MapFixture::new(5, 5);

        let result = AStarPathFinder::find(fixture.start(), fixture.map.center, &fixture.map);

        assert_eq!(result, PathFindingResult::ClosestPos(fixture.start()));
    }

    #[test]
    fn find_at_goal() {
        let fixture = MapFixture::new(0, 0);

        let result = AStarPathFinder::find(fixture.start(), fixture.map.center, &fixture.map);

        assert_eq!(result, PathFindingResult::PathLength(1));
    }

    #[test]
    fn find_single_path() {
        let fixture = MapFixture::new(X_MIN, 0).tunnel_east();

        let result = AStarPathFinder::find(fixture.start(), fixture.map.center, &fixture.map);

        assert_eq!(result, PathFindingResult::PathLength(1 + -X_MIN as usize));
    }

    #[test]
    fn find_among_many_paths() {
        let fixture = MapFixture::new(X_MIN, 20).clear_walls();

        let result = AStarPathFinder::find(fixture.start(), fixture.map.center, &fixture.map);

        assert_eq!(
            result,
            PathFindingResult::PathLength((fixture.start().y + 1 + -X_MIN) as usize)
        );
    }

    #[test]
    fn find_barred_path() {
        let x_bar = 10;
        let fixture = MapFixture::new(20, 20)
            .clear_walls()
            .with_vertical_barrier(x_bar);

        let result = AStarPathFinder::find(fixture.start(), fixture.map.center, &fixture.map);

        let closest_pos = DungeonPosition::new(x_bar + 1, fixture.map.center.y);
        assert_eq!(result, PathFindingResult::ClosestPos(closest_pos));
    }

    struct MapFixture {
        pub map: DungeonMap,
    }

    impl MapFixture {
        fn new(x: isize, y: isize) -> Self {
            let mut map = DungeonMap::new();

            let start = DungeonPosition::new(x, y);
            map.set_tile_type(&start, TileType::Floor);
            map.player_starting_positions.push(start);

            let center = map.center;
            map.set_tile_type(&center, TileType::Floor);

            Self { map }
        }

        fn clear_walls(mut self) -> Self {
            self.map
                .tiles()
                .map(|t| t.pos)
                .collect::<Vec<_>>()
                .iter()
                .for_each(|pos| {
                    self.map.set_tile_type(&pos, TileType::Floor);
                });

            self
        }

        fn start(&self) -> DungeonPosition {
            self.map.player_starting_positions[0]
        }

        fn tunnel_east(mut self) -> Self {
            for x in self.start().x..=self.map.center.x {
                let pos = DungeonPosition::new(x, self.start().y);
                self.map.set_tile_type(&pos, TileType::Floor);
            }

            self
        }

        fn with_vertical_barrier(mut self, x: isize) -> Self {
            for y in Y_MIN..=Y_MAX {
                let pos = DungeonPosition::new(x, y);
                self.map.set_tile_type(&pos, TileType::Wall);
            }

            self
        }
    }
}
