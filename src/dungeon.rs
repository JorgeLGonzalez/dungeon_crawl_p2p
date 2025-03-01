mod components;
mod config;
mod dungeon_map {
    mod a_star {
        mod a_star_node;
        mod a_star_path_finder;

        pub(super) use a_star_path_finder::AStarPathFinder;

        use super::*;
        use a_star_node::AStarNode;
    }
    mod cell_automata {
        mod cell_automata_builder;
        mod cell_grower;
        mod tunneler;

        pub use cell_automata_builder::CellAutomataBuilder;

        use super::*;
        use cell_grower::CellGrower;
        use tunneler::Tunneler;
    }

    mod drunkards_walk {
        mod config;
        mod drunkards_walk_builder;

        pub use config::DrunkardsWalkConfig;
        pub use drunkards_walk_builder::DrunkardsWalkBuilder;

        use super::*;
    }

    mod dungeon_corner;
    mod dungeon_map;
    mod dungeon_position;
    mod dungeon_tile;

    mod random_rooms {
        mod random_rooms_builder;
        mod room;

        pub use random_rooms_builder::RandomRoomsBuilder;

        use super::*;
        use room::Room;
    }

    pub use dungeon_map::DungeonMap;
    pub use dungeon_position::DungeonPosition;

    pub(super) use cell_automata::CellAutomataBuilder;
    pub(super) use drunkards_walk::{DrunkardsWalkBuilder, DrunkardsWalkConfig};
    pub(super) use dungeon_tile::{DungeonTile, TileType};
    pub(super) use random_rooms::RandomRoomsBuilder;

    use super::config::*;
    use a_star::AStarPathFinder;
    use dungeon_corner::DungeonCorner;
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
use dungeon_map::{
    CellAutomataBuilder, DrunkardsWalkBuilder, DrunkardsWalkConfig, RandomRoomsBuilder, TileType,
};
use events::*;
use reveal_map::reveal_map;
use spawn_dungeon::spawn_dungeon;
use zoom::zoom;
