use super::*;
use crate::prelude::*;
use rand::prelude::*;

pub struct DrunkardsWalkBuilder {
    config: DrunkardsWalkConfig,
    map: DungeonMap,
    min_floor_count: usize,
}

impl DrunkardsWalkBuilder {
    pub fn build(config: DrunkardsWalkConfig, rng: &mut RandomGenerator) -> DungeonMap {
        info!("Building drunkards walk dungeon.");

        let min_floor_count = (MAP_WIDTH * MAP_HEIGHT) * config.percent_floor / 100;

        Self {
            config,
            map: DungeonMap::new(),
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
        self.map.item_positions = self
            .map
            .spawnable_positions()
            .map(ItemPosition::new)
            .choose_multiple(rng, self.config.num_items);

        self
    }

    fn add_monsters(mut self, rng: &mut RandomGenerator) -> Self {
        self.map.monster_starting_positions = self
            .map
            .spawnable_positions()
            .map(MonsterPosition::new)
            .choose_multiple(rng, self.config.num_monsters);

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
        let random_step = match rng.gen_range(0..4) {
            0 => DungeonPosition::new(pos.x - 1, pos.y),
            1 => DungeonPosition::new(pos.x + 1, pos.y),
            2 => DungeonPosition::new(pos.x, pos.y - 1),
            3 => DungeonPosition::new(pos.x, pos.y + 1),
            _ => unreachable!(),
        };

        let in_bounds =
            |p: DungeonPosition| p.x > X_MIN && p.x < X_MAX && p.y > Y_MIN && p.y < Y_MAX;

        in_bounds(random_step).then_some(random_step)
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

#[cfg(test)]
mod tests {
    use super::{reachability::AStarPathFinder, *};
    use rstest::rstest;

    #[test]
    fn build() {
        let config = DrunkardsWalkConfig::default();
        let mut rng = RandomGenerator::new();
        let percent_floor = config.percent_floor;

        let map = DrunkardsWalkBuilder::build(config, &mut rng);

        let tile_count = map.tiles().count();
        assert_eq!(tile_count, MAP_WIDTH * MAP_HEIGHT);
        let floor_count = map
            .tiles()
            .filter(|t| t.tile_type == TileType::Floor)
            .count();
        let expected = tile_count * percent_floor / 100;
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

        let map = DrunkardsWalkBuilder::build(config, &mut rng);

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
    fn player_can_reach_center() {
        let mut rng = RandomGenerator::new();

        for attempt in 1..=10 {
            let map = DrunkardsWalkBuilder::build(DrunkardsWalkConfig::default(), &mut rng);

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

        let map = DrunkardsWalkBuilder::build(DrunkardsWalkConfig::default(), &mut rng);

        assert_eq!(map.item_positions.len(), num_items);
    }

    #[test]
    fn add_monsters() {
        let config = DrunkardsWalkConfig::default();
        let num_monsters = config.num_monsters;
        let mut rng = RandomGenerator::new();

        let map = DrunkardsWalkBuilder::build(DrunkardsWalkConfig::default(), &mut rng);

        assert_eq!(map.monster_starting_positions.len(), num_monsters);
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
