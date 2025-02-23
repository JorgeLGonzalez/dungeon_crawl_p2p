use super::{player_start::PlayerStart, *};
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
        .set_center()
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
        let quadrant = match rng.gen_range(0..4) {
            0 => DungeonCorner::BottomLeft,
            1 => DungeonCorner::BottomRight,
            2 => DungeonCorner::TopLeft,
            3 => DungeonCorner::TopRight,
            _ => unreachable!(),
        };

        PlayerStart::add_starting_position(&mut self.map, 0, quadrant);

        if config::GAME_MODE != GameMode::SinglePlayer {
            PlayerStart::add_starting_position(&mut self.map, 1, quadrant.opposite());
        }

        self
    }

    /// Reset the map center to the floor tile nearest the absolute center.
    fn set_center(mut self) -> Self {
        self.map.center = self.map.find_nearest_floor_tile(self.map.center, 1);

        self
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
