use super::dungeon_position::DungeonPosition;

pub struct DungeonTile {
    pub pos: DungeonPosition,
    pub tile_type: TileType,
}

impl DungeonTile {
    pub fn new(pos: DungeonPosition, tile_type: TileType) -> Self {
        Self { pos, tile_type }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TileType {
    #[allow(dead_code)]
    Exit,
    Floor,
    Wall,
}
