use super::{dungeon_position::DungeonPosition, dungeon_tile::DungeonTile, TileType};
use crate::resources::config::{MAP_HEIGHT, MAP_WIDTH};

const NUM_TILES: usize = MAP_WIDTH * MAP_HEIGHT;

pub struct DungeonMap {
    pub tiles: Vec<TileType>,
}

impl DungeonMap {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Wall; NUM_TILES],
        }
    }

    pub fn is_valid_position(&self, pos: &DungeonPosition) -> bool {
        MapPos::from_dungeon_pos(pos).is_valid()
    }

    pub fn set_tile_type(&mut self, pos: &DungeonPosition, tile_type: TileType) {
        self.tiles[MapPos::from_dungeon_pos(pos).to_idx()] = tile_type;
    }

    pub fn tiles(&self) -> impl Iterator<Item = DungeonTile> + use<'_> {
        self.tiles
            .iter()
            .enumerate()
            .map(move |(idx, tile_type)| DungeonTile::new(self.idx_to_position(idx), *tile_type))
    }

    fn idx_to_position(&self, index: usize) -> DungeonPosition {
        assert!(index < self.tiles.len());

        let idx = index as isize;
        const W: isize = MAP_WIDTH as isize;
        const H: isize = MAP_HEIGHT as isize;
        let x = (idx % W) - (W / 2);
        let y = (idx / W) - (H / 2);

        DungeonPosition::new(x, y)
    }
}

struct MapPos {
    x: usize,
    y: usize,
}

impl MapPos {
    pub fn from_dungeon_pos(pos: &DungeonPosition) -> Self {
        let x = (pos.x + (MAP_WIDTH as isize / 2)) as usize;
        let y = (pos.y + (MAP_HEIGHT as isize / 2)) as usize;

        Self { x, y }
    }

    pub fn is_valid(&self) -> bool {
        self.x < MAP_WIDTH && self.y < MAP_HEIGHT
    }

    pub fn to_idx(&self) -> usize {
        self.y * MAP_WIDTH + self.x
    }
}
