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
        let mut finder = Self::new(goal, start);

        finder.find_path(map);

        finder
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

    fn find_path(&mut self, map: &DungeonMap) {
        while let Some(current) = self.open_set.pop() {
            if current.pos == self.goal {
                self.closest_distance = 0;
                self.closest_pos = current.pos;

                return;
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
