pub mod config;
mod dungeon_map;
mod player_inputs;
mod random_generator;

use bevy::prelude::Resource;
use bevy_matchbox::prelude::MatchboxSocket;
pub use dungeon_map::{DungeonMap, DungeonPosition, RandomRoomsBuilder, TileType};
pub use player_inputs::InputDirection;
pub use random_generator::RandomGenerator;

#[derive(Resource)]
pub struct MatchboxSocketResource(pub MatchboxSocket);
