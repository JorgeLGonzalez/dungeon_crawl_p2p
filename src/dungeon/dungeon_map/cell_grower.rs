use super::*;

/// Helper for CellAutomataBuilder
pub(super) struct CellGrower {
    map: DungeonMap,
}

impl CellGrower {
    /// Grow each non-perimeter tile based on the number of adjacent wall tiles.
    pub fn grow(map: DungeonMap) -> DungeonMap {
        let mut grower = Self { map };

        for _ in 0..10 {
            grower.generation().iter().for_each(|tile| {
                grower.map.set_tile_type(&tile.pos, tile.tile_type);
            });
        }

        grower.map
    }

    /// Count the number of wall tiles adjacent to the given position.
    /// The position itself is not counted.
    fn count_adjacent_walls(&self, pos: &DungeonPosition) -> usize {
        const ADJACENT_POSITIONS: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let is_wall =
            |neighbor: &DungeonPosition| self.map.get_tile_type(neighbor) == TileType::Wall;

        pos.perimeter(1).filter(is_wall).count()
    }

    /// Perform one generation of growth
    fn generation(&self) -> Vec<DungeonTile> {
        self.map
            .tiles()
            .filter_map(|t| self.maybe_convert_tile(t))
            .collect::<Vec<_>>()
    }

    /// Convert to wall if surrounded by too many walls (or no walls at all).
    /// Otherwise convert to floor.
    /// Perimeter tiles are not converted.
    fn maybe_convert_tile(&self, tile: DungeonTile) -> Option<DungeonTile> {
        if tile.pos.at_perimeter() {
            return None;
        }

        let adjacent_wall_num = self.count_adjacent_walls(&tile.pos);
        let new_type = if adjacent_wall_num > 4 || adjacent_wall_num == 0 {
            TileType::Wall
        } else {
            TileType::Floor
        };

        (new_type != tile.tile_type).then(|| DungeonTile::new(tile.pos, new_type))
    }
}
