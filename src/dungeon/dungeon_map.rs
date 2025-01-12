mod dungeon_map;
mod dungeon_position;
mod dungeon_tile;
mod random_rooms_builder;
mod room;

pub use dungeon_map::DungeonMap;
pub use dungeon_position::DungeonPosition;
pub use random_rooms_builder::RandomRoomsBuilder;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TileType {
    #[allow(dead_code)]
    Exit,
    Floor,
    Wall,
}
