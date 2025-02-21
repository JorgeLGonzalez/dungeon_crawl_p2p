use super::{DungeonMap, MapPos};
use crate::{
    common::RandomGenerator,
    config,
    dungeon::{DungeonPosition, TileType},
};
use rand::prelude::*;

pub struct CellAutomataBuilder {
    map: DungeonMap,
}

impl CellAutomataBuilder {
    pub fn build(rng: &mut RandomGenerator) -> DungeonMap {
        Self {
            map: DungeonMap::new(),
        }
        .randomize_tiles(rng)
        .smoothen()
        .add_player_starting_positions()
        .add_items(rng)
        .add_monster_starting_positions(rng)
        .map
    }

    fn add_items(mut self, rng: &mut RandomGenerator) -> Self {
        self.map.item_positions = self
            .map
            .spawnable_positions()
            .choose_multiple(rng, config::NUM_ITEMS);

        self
    }

    fn add_monster_starting_positions(mut self, rng: &mut RandomGenerator) -> Self {
        self.map.monster_starting_positions = self
            .map
            .spawnable_positions()
            .choose_multiple(rng, config::NUM_MONSTERS);

        self
    }

    fn add_player_starting_positions(mut self) -> Self {
        let pos = self
            .map
            .tiles()
            .find(|t| t.tile_type == TileType::Floor)
            .unwrap()
            .pos;

        self.map.player_starting_positions.push(pos);

        // if config::GAME_MODE != GameMode::SinglePlayer {
        //     self.map
        //         .player_starting_positions
        //         .push(self.rooms[1].center());
        // }

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
        for y in 1..config::MAP_HEIGHT - 1 {
            for x in 1..config::MAP_WIDTH - 1 {
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

            for y in 1..config::MAP_HEIGHT - 1 {
                for x in 1..config::MAP_WIDTH - 1 {
                    let pos = MapPos::new(x, y).to_dungeon_pos();
                    let neighbors = self.count_adjacent_walls(&pos);

                    let tile = if neighbors > 4 || neighbors == 0 {
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
}
