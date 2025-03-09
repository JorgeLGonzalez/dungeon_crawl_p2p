use super::{position::MonsterPosition, *};
use crate::common::RandomGenerator;
use bevy::prelude::*;
use rand::seq::IteratorRandom;

const NUM_TILES: usize = MAP_WIDTH * MAP_HEIGHT;

#[derive(Resource)]
pub struct DungeonMap {
    /// Floor tile closest to the center of the dungeon where we put the exit
    /// or amulet.
    pub center: DungeonPosition,
    pub level: usize,
    pub player_starting_positions: Vec<DungeonPosition>,

    item_positions: Vec<ItemPosition>,
    monster_starting_positions: Vec<MonsterPosition>,
    tiles: Vec<TileType>,
}

impl DungeonMap {
    pub fn new(level: usize) -> Self {
        Self {
            center: DungeonPosition::new(0, 0),
            item_positions: vec![],
            level,
            monster_starting_positions: vec![],
            player_starting_positions: vec![],
            tiles: vec![TileType::Wall; NUM_TILES],
        }
    }

    /// Add items to random spawnable positions in the dungeon. (Replaces any
    /// existing item positions.)
    pub fn add_items(&mut self, count: usize, rng: &mut RandomGenerator) {
        self.item_positions = self
            .spawnable_positions()
            .map(ItemPosition::new)
            .choose_multiple(rng, count);
    }

    /// Add monsters to random spawnable positions in the dungeon. (Replaces any
    /// existing monster positions.)
    pub fn add_monsters(&mut self, count: usize, rng: &mut RandomGenerator) {
        self.monster_starting_positions = self
            .spawnable_positions()
            .map(MonsterPosition::new)
            .choose_multiple(rng, count);
    }

    pub fn add_one_item(&mut self, item: ItemPosition) {
        self.item_positions.push(item);
        trace!("{item} placed at {}", item.pos);
    }

    pub fn add_one_monster(&mut self, monster: MonsterPosition) {
        self.monster_starting_positions.push(monster);
        trace!("{monster} placed at {}", monster.pos);
    }

    /// Returns an [`IRect`] that encompasses the entire dungeon map.
    pub fn bounds(&self) -> IRect {
        IRect::new(X_MIN as i32, Y_MIN as i32, X_MAX as i32, Y_MAX as i32)
    }

    /// Returns an [`IRect`] that encompasses the entire dungeon map excluding
    /// the outermost walls.
    pub fn bounds_inner(&self) -> IRect {
        self.bounds().inflate(-1)
    }

    /// Remove any monsters or items slated for the tiles encompassed by the vault.
    /// (Players are NOT removed.)
    pub fn clear_area(&mut self, area: IRect) {
        self.item_positions
            .retain(|&pos| !area.contains(pos.into()));
        self.monster_starting_positions
            .retain(|&pos| !area.contains(pos.into()));
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

    pub fn item_positions(&self) -> impl Iterator<Item = ItemPosition> + use<'_> {
        self.item_positions.iter().copied()
    }

    pub fn monster_starting_positions(&self) -> impl Iterator<Item = MonsterPosition> + use<'_> {
        self.monster_starting_positions.iter().copied()
    }

    pub fn set_tile_type(&mut self, pos: &DungeonPosition, tile_type: TileType) {
        self.tiles[MapPos::from(pos).to_idx()] = tile_type;
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

    /// Returns an iterator over all spawnable positions for monsters and items.
    /// Spawnable positions are floor tiles that are outside the player's safety
    /// radius and exclude the dungeon "center".
    fn spawnable_positions(&self) -> impl Iterator<Item = DungeonPosition> + use<'_> {
        self.tiles()
            .filter(|t| t.tile_type == TileType::Floor)
            .filter(|t| t.pos != self.center)
            .filter(|t| self.far_from_players(t.pos))
            .map(|t| t.pos)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds() {
        let map = DungeonMap::new(1);

        let bounds = map.bounds();

        assert_eq!(bounds.min.x, X_MIN as i32, "min x");
        assert_eq!(bounds.min.y, Y_MIN as i32, "min y");
        assert_eq!(bounds.max.x, X_MAX as i32, "max x");
        assert_eq!(bounds.max.y, Y_MAX as i32, "max y");
        assert_eq!(
            map.idx_to_position(NUM_TILES - 1),
            DungeonPosition::new(X_MAX, Y_MAX)
        );
        assert_eq!(bounds.width(), MAP_WIDTH as i32 - 1, "width");
        assert_eq!(bounds.height(), MAP_HEIGHT as i32 - 1, "height");
    }
}
