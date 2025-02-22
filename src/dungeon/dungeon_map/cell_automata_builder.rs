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

    fn add_player_starting_positions(mut self, rng: &mut RandomGenerator) -> Self {
        let corner = match rng.gen_range(0..4) {
            0 => MapPos::new(1, 1),
            1 => MapPos::new(MAP_WIDTH - 2, 1),
            2 => MapPos::new(1, MAP_HEIGHT - 2),
            _ => MapPos::new(MAP_WIDTH - 2, MAP_HEIGHT - 2),
        };

        let radius = 1;
        let pos = self.find_nearest_floor_tile(corner.to_dungeon_pos(), radius);
        self.map.player_starting_positions.push(pos);

        if config::GAME_MODE != GameMode::SinglePlayer {
            let opposite_x = if corner.x == 1 { MAP_WIDTH - 2 } else { 1 };
            let opposite_y = if corner.y == 1 { MAP_HEIGHT - 2 } else { 1 };
            let opposite = MapPos::new(opposite_x, opposite_y);
            let pos = self.find_nearest_floor_tile(opposite.to_dungeon_pos(), radius);
            self.map.player_starting_positions.push(pos);
        }

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

    /// Smooth out the randomly assigned tiles by converting tiles to floor unless
    /// surrounded by too many walls (or no walls at all).
    fn grow_cells(mut self) -> Self {
        self.map = CellGrower::grow(self.map);

        self
    }

    fn find_nearest_floor_tile(&self, origin: DungeonPosition, radius: isize) -> DungeonPosition {
        if radius == 1 && self.map.get_tile_type(&origin) == TileType::Floor {
            return origin;
        }

        for iy in -radius..=radius {
            for ix in -radius..=radius {
                let pos = DungeonPosition::new(origin.x + ix, origin.y + iy);
                if !self.map.is_valid_position(&pos) {
                    continue;
                }
                let tile = self.map.get_tile_type(&pos);
                if tile == TileType::Floor {
                    return pos;
                }
            }
        }

        // Expand the search radius and try again
        self.find_nearest_floor_tile(origin, radius + 1)
    }
}
