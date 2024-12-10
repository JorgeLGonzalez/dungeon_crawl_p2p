pub mod config;
mod dungeon_map;

use bevy::prelude::Resource;
use bevy_matchbox::{prelude::SingleChannel, MatchboxSocket};
pub use dungeon_map::{RandomRoomsBuilder, TileType};

#[derive(Resource)]
pub struct MatchboxSocketResource(pub MatchboxSocket<SingleChannel>);

#[derive(Clone, Copy, Resource)]
pub struct SessionSeed(pub u64);
