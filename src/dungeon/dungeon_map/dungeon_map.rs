use super::*;
use bevy::prelude::Resource;

const NUM_TILES: usize = MAP_WIDTH * MAP_HEIGHT;

#[derive(Resource)]
pub struct DungeonMap {
    /// Floor tile closest to the center of the dungeon where we put the exit
    /// or amulet.
    /// TODO Not set for random rooms.
    pub center: DungeonPosition,
    pub item_positions: Vec<DungeonPosition>,
    pub monster_starting_positions: Vec<DungeonPosition>,
    pub player_starting_positions: Vec<DungeonPosition>,
    tiles: Vec<TileType>,
}

impl DungeonMap {
    pub fn new() -> Self {
        Self {
            center: DungeonPosition::new(0, 0),
            item_positions: vec![],
            monster_starting_positions: vec![],
            player_starting_positions: vec![],
            tiles: vec![TileType::Wall; NUM_TILES],
        }
    }

    /// Find the nearest floor tile to the given origin, within the given radius.
    /// If no floor tile is found within the radius, recursively search with an
    /// increased radius.
    pub fn find_nearest_floor_tile(
        &self,
        origin: DungeonPosition,
        radius: isize,
    ) -> DungeonPosition {
        assert!(radius > 0 && radius < 10);
        if radius == 1 && self.get_tile_type(&origin) == TileType::Floor {
            return origin;
        }

        origin
            .perimeter(radius)
            .filter(|pos| self.is_valid_position(pos))
            .find(|pos| self.get_tile_type(pos) == TileType::Floor)
            .unwrap_or_else(|| self.find_nearest_floor_tile(origin, radius + 1))
    }

    pub fn get_tile_type(&self, pos: &DungeonPosition) -> TileType {
        self.tiles[MapPos::from(pos).to_idx()]
    }

    pub fn is_valid_position(&self, pos: &DungeonPosition) -> bool {
        MapPos::from(pos).is_valid()
    }

    pub fn set_tile_type(&mut self, pos: &DungeonPosition, tile_type: TileType) {
        self.tiles[MapPos::from(pos).to_idx()] = tile_type;
    }

    /// Returns an iterator over all spawnable positions for monsters and items.
    /// Spawnable positions are floor tiles that are outside the player's safety
    /// radius.
    pub fn spawnable_positions(&self) -> impl Iterator<Item = DungeonPosition> + use<'_> {
        self.tiles()
            .filter(|t| t.tile_type == TileType::Floor)
            .filter(|t| self.far_from_players(t.pos))
            .map(|t| t.pos)
    }

    pub fn tiles(&self) -> impl Iterator<Item = DungeonTile> + use<'_> {
        self.tiles
            .iter()
            .enumerate()
            .map(move |(idx, tile_type)| DungeonTile::new(self.idx_to_position(idx), *tile_type))
    }

    fn far_from_players(&self, pos: DungeonPosition) -> bool {
        self.player_starting_positions
            .iter()
            .all(|p| p.distance(pos).abs() > SAFETY_RADIUS)
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

/// A position in the dungeon map where the coordinate system origin is at the
/// top left, unlike the DungeonPosition's coordinate system's origin which is
/// at the center.
#[derive(Clone, Copy)]
struct MapPos {
    pub x: usize,
    pub y: usize,
}

impl MapPos {
    pub fn is_valid(&self) -> bool {
        self.x < MAP_WIDTH && self.y < MAP_HEIGHT
    }

    pub fn to_idx(&self) -> usize {
        self.y * MAP_WIDTH + self.x
    }
}

impl From<&DungeonPosition> for MapPos {
    fn from(pos: &DungeonPosition) -> Self {
        let x = (pos.x + (MAP_WIDTH as isize / 2)) as usize;
        let y = (pos.y + (MAP_HEIGHT as isize / 2)) as usize;

        Self { x, y }
    }
}
