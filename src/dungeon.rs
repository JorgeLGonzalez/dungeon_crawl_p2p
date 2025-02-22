mod components;
mod config;

mod dungeon_map {
    mod cell_automata_builder;
    mod cell_grower;
    mod dungeon_map;
    mod dungeon_position;
    mod dungeon_tile;
    mod random_rooms_builder;
    mod room;

    pub use dungeon_map::DungeonMap;
    pub use dungeon_position::DungeonPosition;

    pub(super) use cell_automata_builder::CellAutomataBuilder;
    pub(super) use dungeon_map::MapPos;
    pub(super) use dungeon_tile::{DungeonTile, TileType};
    pub(super) use random_rooms_builder::RandomRoomsBuilder;
    pub(super) use room::Room;

    use super::config::*;
    use cell_grower::CellGrower;
}
mod events;
mod illuminator;
mod plugin;
mod reveal_map;
mod spawn_dungeon;
mod zoom;

pub use components::{FloorTile, WallTile};
pub use config::{NUM_MONSTERS, TILE_HEIGHT, TILE_WIDTH, VIEWPORT_HEIGHT};
pub use dungeon_map::{DungeonMap, DungeonPosition};
pub use events::{RevealDungeonEvent, ZoomEvent};
pub use illuminator::{FloorQuery, Illuminator, PlayerQuery};
pub use plugin::{DungeonCoreSet, DungeonPlugin, SpawnDungeonSet};

use components::*;
use config::*;
use dungeon_map::{CellAutomataBuilder, RandomRoomsBuilder, TileType};
use events::*;
use reveal_map::reveal_map;
use spawn_dungeon::spawn_dungeon;
use zoom::zoom;
