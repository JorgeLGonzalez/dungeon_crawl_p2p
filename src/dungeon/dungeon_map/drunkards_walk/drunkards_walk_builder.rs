use super::*;
use crate::prelude::*;

pub struct DrunkardsWalkBuilder {
    config: DrunkardsWalkConfig,
    inner_bounds: IRect,
    map: DungeonMap,
    min_floor_count: usize,
}

impl DrunkardsWalkBuilder {
    pub fn build(
        config: DrunkardsWalkConfig,
        level: usize,
        rng: &mut RandomGenerator,
    ) -> DungeonMap {
        info!("Building drunkards walk dungeon.");

        let map = DungeonMap::new(level);
        let bounds = map.bounds();
        let min_floor_count =
            (bounds.width() * bounds.height()) as usize * config.percent_floor / 100;

        Self {
            config,
            inner_bounds: map.bounds_inner(),
            map,
            min_floor_count,
        }
        .add_player_positions(rng)
        .tunnel(rng)
        .connect_players()
        .add_items(rng)
        .add_monsters(rng)
        .map
    }

    fn add_items(mut self, rng: &mut RandomGenerator) -> Self {
        self.map.add_items(self.config.num_items, rng);

        self
    }

    fn add_monsters(mut self, rng: &mut RandomGenerator) -> Self {
        self.map.add_monsters(self.config.num_monsters, rng);

        self
    }

    /// Add players to opposite corners of the dungeon.
    fn add_player_positions(mut self, rng: &mut RandomGenerator) -> Self {
        let corner = DungeonCorner::random(rng);
        self.map.player_starting_positions.push(corner.pos());

        if self.config.num_players == 2 {
            self.map
                .player_starting_positions
                .push(corner.opposite().pos());
        }

        self
    }

    /// Ensure both players can reach the center of the dungeon, tunneling if
    /// necessary.
    fn connect_players(mut self) -> Self {
        ReachabilityEnsurer::ensure(
            &Searchers::from_players(&self.map),
            self.map.center,
            &mut self.map,
        );

        self
    }

    /// Drunkenly tunnel from the given position until we either stagger enough
    /// steps or hit the dungeon boundaries too many times.
    fn drunkard(&mut self, start: &DungeonPosition, rng: &mut RandomGenerator) {
        let mut drunkard_pos = start.clone();
        let mut stagger_steps = 0;
        let mut retries = 0;

        while stagger_steps <= MAX_DRUNKARD_STEPS && retries < 10 {
            self.map.set_tile_type(&drunkard_pos, TileType::Floor);

            if let Some(pos) = self.step(drunkard_pos, rng) {
                drunkard_pos = pos;
                stagger_steps += 1;
                retries = 0;
            } else {
                retries += 1;
            }
        }

        if retries >= 10 {
            warn!("Drunkard at {drunkard_pos} hit max stagger retries");
        }
    }

    /// Check if we have enough floor tiles.
    fn insufficient_floor(&self) -> bool {
        self.map
            .tiles()
            .filter(|t| t.tile_type == TileType::Floor)
            .count()
            < self.min_floor_count
    }

    /// Take a random step in one of the four cardinal directions. Return None
    /// if we hit the dungeon boundaries.
    fn step(&self, pos: DungeonPosition, rng: &mut RandomGenerator) -> Option<DungeonPosition> {
        let random_step =
            DungeonPosition::from_vec2(pos.to_vec2() + DIRECTIONS[rng.gen_range(0..4)]);

        self.inner_bounds
            .contains(random_step.as_ivec2())
            .then_some(random_step)
    }

    /// Repeatedly drunkenly tunnel first starting from the center and each
    /// player starting position. Repeat from random positions until we have
    /// enough floor tiles.
    fn tunnel(mut self, rng: &mut RandomGenerator) -> Self {
        let center = self.map.center;
        self.drunkard(&center, rng);
        for pos in self.map.player_starting_positions.clone() {
            self.drunkard(&pos, rng);
        }

        while self.insufficient_floor() {
            let random_pos = DungeonPosition::new(
                rng.gen_range(X_MIN + 1..X_MAX - 1),
                rng.gen_range(Y_MIN + 1..Y_MAX - 1),
            );

            self.drunkard(&random_pos, rng);
        }

        self
    }
}

/// Left, right, up, down
const DIRECTIONS: [Vec2; 4] = [Vec2::NEG_X, Vec2::X, Vec2::NEG_Y, Vec2::Y];

#[cfg(test)]
mod tests {
    use super::{reachability::AStarPathFinder, *};
    use rstest::rstest;

    #[test]
    fn build() {
        let config = DrunkardsWalkConfig::default();
        let mut rng = RandomGenerator::new();
        let percent_floor = config.percent_floor;

        let map = DrunkardsWalkBuilder::build(config, 1, &mut rng);

        let tile_count = map.tiles().count();
        assert_eq!(tile_count, MAP_WIDTH * MAP_HEIGHT);
        let inner_area = (map.bounds_inner().width() * map.bounds_inner().height()) as usize;
        let floor_count = map
            .tiles()
            .filter(|t| t.tile_type == TileType::Floor)
            .count();
        let expected = inner_area * percent_floor / 100;
        assert!(
            floor_count >= expected,
            "actual floor={floor_count} expected={expected}"
        );
    }

    #[rstest]
    #[case::one_player(1)]
    #[case::two_players(2)]
    fn add_player_position(#[case] num_players: usize) {
        let config = DrunkardsWalkConfig {
            num_players,
            ..default()
        };
        let mut rng = RandomGenerator::new();

        let map = DrunkardsWalkBuilder::build(config, 1, &mut rng);

        assert_eq!(
            map.player_starting_positions.len(),
            num_players,
            "player positions"
        );
        map.player_starting_positions
            .iter()
            .enumerate()
            .for_each(|(player_id, pos)| {
                let edge_distance = edge_distance(pos);
                assert!(
                    edge_distance < 4,
                    "player {player_id} too far from edge at {edge_distance}"
                );
            });
    }

    #[test]
    fn players_on_opposite_corners() {
        let mut rng = RandomGenerator::new();

        let map = DrunkardsWalkBuilder::build(
            DrunkardsWalkConfig {
                num_players: 2,
                ..default()
            },
            1,
            &mut rng,
        );

        let pos1 = map.player_starting_positions[0];
        let pos2 = map.player_starting_positions[1];
        let distance = pos1.distance(pos2);
        assert!(
            distance > 80.,
            "players too close together at distance {distance}"
        );
    }

    #[test]
    fn walled_perimeter() {
        let config = DrunkardsWalkConfig::default();
        let mut rng = RandomGenerator::new();

        let map = DrunkardsWalkBuilder::build(config, 1, &mut rng);

        let bounds = map.bounds();
        for x in bounds.min.x..=bounds.max.x {
            let bottom = DungeonPosition::new(x as isize, bounds.min.y as isize);
            assert_eq!(
                map.get_tile_type(&bottom),
                TileType::Wall,
                "wall missing at {bottom}"
            );
            let top = DungeonPosition::new(x as isize, bounds.max.y as isize);
            assert_eq!(
                map.get_tile_type(&top),
                TileType::Wall,
                "wall missing at {top}"
            );
        }
        for y in bounds.min.y..=bounds.max.y {
            let left = DungeonPosition::new(bounds.min.x as isize, y as isize);
            assert_eq!(
                map.get_tile_type(&left),
                TileType::Wall,
                "wall missing at {left}"
            );
            let right = DungeonPosition::new(bounds.max.x as isize, y as isize);
            assert_eq!(
                map.get_tile_type(&right),
                TileType::Wall,
                "wall missing at {right}"
            );
        }
    }

    #[test]
    fn player_can_reach_center() {
        let mut rng = RandomGenerator::new();

        for attempt in 1..=10 {
            let map = DrunkardsWalkBuilder::build(DrunkardsWalkConfig::default(), 1, &mut rng);

            let player_pos = map.player_starting_positions[0];
            let finder = AStarPathFinder::find(player_pos, map.center, &map);

            assert!(
                finder.path_found(),
                "player unable to reach center on attempt {attempt}"
            );
        }
    }

    #[test]
    fn add_items() {
        let config = DrunkardsWalkConfig::default();
        let num_items = config.num_items;
        let mut rng = RandomGenerator::new();

        let map = DrunkardsWalkBuilder::build(DrunkardsWalkConfig::default(), 1, &mut rng);

        assert_eq!(map.item_positions().count(), num_items);
    }

    #[test]
    fn add_monsters() {
        let config = DrunkardsWalkConfig::default();
        let num_monsters = config.num_monsters;
        let mut rng = RandomGenerator::new();

        let map = DrunkardsWalkBuilder::build(DrunkardsWalkConfig::default(), 1, &mut rng);

        assert_eq!(map.monster_starting_positions().count(), num_monsters);
    }

    fn edge_distance(pos: &DungeonPosition) -> usize {
        [
            pos.distance(DungeonPosition::new(X_MIN, Y_MAX)),
            pos.distance(DungeonPosition::new(X_MAX, Y_MAX)),
            pos.distance(DungeonPosition::new(X_MAX, Y_MIN)),
            pos.distance(DungeonPosition::new(X_MIN, Y_MIN)),
        ]
        .iter()
        .map(|d| d.round().abs() as usize)
        .min()
        .unwrap()
    }
}
