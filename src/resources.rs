pub mod config;
mod dungeon_map;
mod monster_move_tracker;
mod player_inputs;
mod random_generator;

use bevy::prelude::Resource;
use bevy_matchbox::prelude::MatchboxSocket;
pub use dungeon_map::{DungeonMap, DungeonPosition, RandomRoomsBuilder, TileType};
pub use monster_move_tracker::*;
pub use player_inputs::PlayerInputCode;
pub use random_generator::RandomGenerator;

#[derive(Resource)]
pub struct MatchboxSocketResource(pub MatchboxSocket);
