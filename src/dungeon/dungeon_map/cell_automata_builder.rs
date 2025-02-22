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
        .smoothen()
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

    /// Count the number of wall tiles adjacent to the given position.
    /// The position itself is not counted.
    fn count_adjacent_walls(&self, pos: &DungeonPosition) -> usize {
        let mut neighbors = 0;
        for iy in -1..=1 {
            for ix in -1..=1 {
                let tile = self
                    .map
                    .get_tile_type(&DungeonPosition::new(pos.x + ix, pos.y + iy));
                if !(ix == 0 && iy == 0) && tile == TileType::Wall {
                    neighbors += 1;
                }
            }
        }

        neighbors
    }

    /// Randomly assign tiles within the map, slightly favoring floors.
    /// Map perimeter is left as walls.
    fn randomize_tiles(mut self, rng: &mut RandomGenerator) -> Self {
        for y in 1..MAP_HEIGHT - 1 {
            for x in 1..MAP_WIDTH - 1 {
                let tile = if rng.gen_range(0..100) < 45 {
                    TileType::Wall
                } else {
                    TileType::Floor
                };

                self.map
                    .set_tile_type(&MapPos::new(x, y).to_dungeon_pos(), tile);
            }
        }

        self
    }

    /// Smooth out the randomly assigned tiles by converting tiles to floor unless
    /// surrounded by too many walls (or no walls at all).
    fn smoothen(mut self) -> Self {
        for _ in 0..10 {
            let mut tiles_clone = self.map.tiles.clone();

            for y in 1..MAP_HEIGHT - 1 {
                for x in 1..MAP_WIDTH - 1 {
                    let pos = MapPos::new(x, y).to_dungeon_pos();
                    let adjacent_wall_num = self.count_adjacent_walls(&pos);

                    let tile = if adjacent_wall_num > 4 || adjacent_wall_num == 0 {
                        TileType::Wall
                    } else {
                        TileType::Floor
                    };

                    tiles_clone[MapPos::new(x, y).to_idx()] = tile;
                }
            }

            self.map.tiles = tiles_clone;
        }

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
