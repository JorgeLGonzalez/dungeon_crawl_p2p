use super::{AStarNode, DungeonMap, DungeonPosition, TileType};
use bevy::utils::hashbrown::HashMap;
use std::collections::BinaryHeap;

/// A* search path finder used to determine whether a path exists between two
/// positions (e.g. player and dungeon center).
pub struct AStarPathFinder {
    /// Node pair where the key is the position reached from the value that was
    /// reached from the start position. See
    /// [`calculate_path_length`](Self::calculate_path_length) for how to retrace
    /// the path from the goal back to the start.
    came_from: HashMap<DungeonPosition, DungeonPosition>,
    /// Closest distance to the goal found so far.
    closest_distance: usize,
    /// Closest position to the goal found so far.
    closest_pos: DungeonPosition,
    goal: DungeonPosition,
    /// Cost of reaching the position from the start position.
    partial_costs: HashMap<DungeonPosition, usize>,
    /// Open set (ordered by cost) of nodes to be evaluated.
    open_set: BinaryHeap<AStarNode>,
}

impl AStarPathFinder {
    #[allow(dead_code, reason = "for testing")]
    pub fn calculate_path_length(&self) -> usize {
        let mut path_len = 1;
        let mut current = self.goal;

        while let Some(&prev) = self.came_from.get(&current) {
            path_len += 1;
            current = prev;
        }

        path_len
    }

    pub fn closest_position(&self) -> DungeonPosition {
        self.closest_pos
    }

    pub fn path_found(&self) -> bool {
        self.closest_distance == 0
    }

    pub fn find(start: DungeonPosition, goal: DungeonPosition, map: &DungeonMap) -> Self {
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
            partial_costs: node_costs,
            open_set,
        }
    }

    /// Enqueue the given neighbor for evaluation and update associated data
    /// structures.
    fn enqueue_neighbor(
        &mut self,
        current_pos: DungeonPosition,
        neighbor: DungeonPosition,
        tentative_cost: usize,
    ) {
        self.came_from.insert(neighbor, current_pos);
        self.partial_costs.insert(neighbor, tentative_cost);
        self.open_set.push(AStarNode::new(
            neighbor,
            tentative_cost + neighbor.manhattan_distance(self.goal),
        ));
    }

    fn find_path(mut self, map: &DungeonMap) -> Self {
        while let Some(current) = self.open_set.pop() {
            if current.pos == self.goal {
                return self.reached_goal();
            }

            self.neighbors(current.pos, map)
                .into_iter()
                .for_each(|neighbor| {
                    self.update_closest(current.pos);

                    let tentative_cost = self.tentative_cost(current.pos);
                    let neighbor_cost = self.neighbor_partial_cost(neighbor);
                    if tentative_cost < neighbor_cost {
                        self.enqueue_neighbor(current.pos, neighbor, tentative_cost);
                    }
                });
        }

        self
    }

    /// Returns the neighbors of the given position that are valid and not walls.
    fn neighbors(&self, pos: DungeonPosition, map: &DungeonMap) -> Vec<DungeonPosition> {
        [
            DungeonPosition::new(pos.x + 1, pos.y),
            DungeonPosition::new(pos.x - 1, pos.y),
            DungeonPosition::new(pos.x, pos.y + 1),
            DungeonPosition::new(pos.x, pos.y - 1),
        ]
        .into_iter()
        .filter(|n| map.is_valid_position(&n) && map.get_tile_type(&n) != TileType::Wall)
        .collect()
    }

    /// The recorded cost of reaching the given position from the start position,
    /// or MAX if the given position has no recorded cost.
    fn neighbor_partial_cost(&self, pos: DungeonPosition) -> usize {
        self.partial_costs.get(&pos).copied().unwrap_or(usize::MAX)
    }

    /// Reached goal so update closest position and distance.
    fn reached_goal(mut self) -> Self {
        self.closest_distance = 0;
        self.closest_pos = self.goal;

        self
    }

    /// Returns the tentative cost of a neighbor of the given position by
    /// adding 1 step to it, or MAX if the given position has no recorded cost.
    /// This is really the minimal possible cost of the partial path to the goal.
    fn tentative_cost(&self, pos: DungeonPosition) -> usize {
        self.partial_costs
            .get(&pos)
            .map(|&c| c + 1)
            .unwrap_or(usize::MAX)
    }

    /// Update closest position and distance if the given position is closer to
    /// the goal than the current closest position.
    fn update_closest(&mut self, pos: DungeonPosition) {
        let distance = pos.manhattan_distance(self.goal);
        if distance < self.closest_distance {
            self.closest_distance = distance;
            self.closest_pos = pos;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::config::{X_MIN, Y_MAX, Y_MIN};

    #[test]
    fn find_no_floors() {
        let fixture = MapFixture::new(5, 5);

        let finder = AStarPathFinder::find(fixture.start(), fixture.map.center, &fixture.map);

        assert!(!finder.path_found());
        assert_eq!(finder.closest_position(), fixture.start());
    }

    #[test]
    fn find_at_goal() {
        let fixture = MapFixture::new(0, 0);

        let finder = AStarPathFinder::find(fixture.start(), fixture.map.center, &fixture.map);

        assert!(finder.path_found());
        assert_eq!(finder.calculate_path_length(), 1);
    }

    #[test]
    fn find_single_path() {
        let fixture = MapFixture::new(X_MIN, 0).tunnel_east();

        let finder = AStarPathFinder::find(fixture.start(), fixture.map.center, &fixture.map);

        assert!(finder.path_found());
        assert_eq!(finder.calculate_path_length(), 1 + -X_MIN as usize);
    }

    #[test]
    fn find_among_many_paths() {
        let fixture = MapFixture::new(X_MIN, 20).clear_walls();

        let finder = AStarPathFinder::find(fixture.start(), fixture.map.center, &fixture.map);

        assert!(finder.path_found());
        assert_eq!(
            finder.calculate_path_length(),
            (fixture.start().y + 1 + -X_MIN) as usize
        );
    }

    #[test]
    fn find_barred_path() {
        let x_bar = 10;
        let fixture = MapFixture::new(20, 20)
            .clear_walls()
            .with_vertical_barrier(x_bar);

        let finder = AStarPathFinder::find(fixture.start(), fixture.map.center, &fixture.map);

        assert!(!finder.path_found());
        let expected = DungeonPosition::new(x_bar + 1, fixture.map.center.y);
        assert_eq!(finder.closest_position(), expected);
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
