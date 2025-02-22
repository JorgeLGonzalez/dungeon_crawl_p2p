use super::*;
use crate::prelude::*;
use rand::prelude::*;

pub struct CellAutomataBuilder {
    map: DungeonMap,
}

impl CellAutomataBuilder {
    pub fn build(rng: &mut RandomGenerator) -> DungeonMap {
        info!("Building cellular automata dungeon.");

        Self {
            map: DungeonMap::new(),
        }
        .randomize_tiles(rng)
        .grow_cells()
        .add_player_starting_positions(rng)
        .add_items(rng)
        .add_monster_starting_positions(rng)
        .map
    }

    fn add_items(mut self, rng: &mut RandomGenerator) -> Self {
        self.map.item_positions = self
            .map
            .spawnable_positions()
            .choose_multiple(rng, NUM_ITEMS);

        self
    }

    fn add_monster_starting_positions(mut self, rng: &mut RandomGenerator) -> Self {
        self.map.monster_starting_positions = self
            .map
            .spawnable_positions()
            .choose_multiple(rng, NUM_MONSTERS);

        self
    }

    /// Randomly assign player starting positions to opposite corners of the
    /// dungeon.
    fn add_player_starting_positions(mut self, rng: &mut RandomGenerator) -> Self {
        let corner = match rng.gen_range(0..4) {
            0 => DungeonCorner::BottomLeft,
            1 => DungeonCorner::BottomRight,
            2 => DungeonCorner::TopLeft,
            3 => DungeonCorner::TopRight,
            _ => unreachable!(),
        };

        let radius = 1;
        let pos = self.find_nearest_floor_tile(corner.pos(), radius);
        self.map.player_starting_positions.push(pos);

        if config::GAME_MODE != GameMode::SinglePlayer {
            let pos = self.find_nearest_floor_tile(corner.opposite().pos(), radius);
            self.map.player_starting_positions.push(pos);
        }

        self
    }

    /// Find the nearest floor tile to the given origin, within the given radius.
    /// If no floor tile is found within the radius, recursively search with an
    /// increased radius.
    fn find_nearest_floor_tile(&self, origin: DungeonPosition, radius: isize) -> DungeonPosition {
        assert!(radius > 0 && radius < 10);
        if radius == 1 && self.map.get_tile_type(&origin) == TileType::Floor {
            return origin;
        }

        origin
            .perimeter(radius)
            .filter(|pos| self.map.is_valid_position(pos))
            .find(|pos| self.map.get_tile_type(pos) == TileType::Floor)
            .unwrap_or_else(|| self.find_nearest_floor_tile(origin, radius + 1))
    }

    /// Smooth out the randomly assigned tiles by converting tiles to floor unless
    /// surrounded by too many walls (or no walls at all).
    fn grow_cells(mut self) -> Self {
        self.map = CellGrower::grow(self.map);

        self
    }

    /// Randomly assign tiles within the map, slightly favoring floors.
    /// Map perimeter is left as walls.
    fn randomize_tiles(mut self, rng: &mut RandomGenerator) -> Self {
        self.map
            .tiles()
            .filter(|t| !t.pos.at_perimeter())
            .filter(|t| rng.gen_range(0..100) >= 55 && t.tile_type == TileType::Wall)
            .map(|t| t.pos)
            .collect::<Vec<_>>()
            .iter()
            .for_each(|pos| {
                self.map.set_tile_type(&pos, TileType::Floor);
            });

        self
    }
}
