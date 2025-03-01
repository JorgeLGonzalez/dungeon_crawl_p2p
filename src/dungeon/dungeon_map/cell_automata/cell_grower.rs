use super::*;

/// Helper for CellAutomataBuilder used to grow cells based on tile neighbors.
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

        let is_wall =
            |neighbor: &DungeonPosition| self.map.get_tile_type(neighbor) == TileType::Wall;

        let adjacent_wall_num = tile.pos.perimeter(1).filter(is_wall).count();
        let new_type = if adjacent_wall_num > 4 || adjacent_wall_num == 0 {
            TileType::Wall
        } else {
            TileType::Floor
        };

        (new_type != tile.tile_type).then(|| DungeonTile::new(tile.pos, new_type))
    }
}
