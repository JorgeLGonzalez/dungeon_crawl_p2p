use super::{dungeon_position::DungeonPosition, TileType};

pub struct DungeonTile {
    pub pos: DungeonPosition,
    pub tile_type: TileType,
}

impl DungeonTile {
    pub fn new(pos: DungeonPosition, tile_type: TileType) -> Self {
        Self { pos, tile_type }
    }
}
