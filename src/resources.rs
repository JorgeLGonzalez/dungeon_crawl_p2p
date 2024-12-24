pub mod config;
mod dungeon_map;
mod monster_move_tracker;
mod player_inputs;
mod random_generator;

use bevy::prelude::Event;
use bevy_ggrs::ggrs::Frame;
pub use dungeon_map::{DungeonMap, DungeonPosition, RandomRoomsBuilder, TileType};
pub use monster_move_tracker::*;
pub use player_inputs::PlayerInputCode;
pub use random_generator::RandomGenerator;

/// Used when an out-of-sync is detected by GGRS. Dispatched by [`crate::systems::handle_ggrs_events`]
/// and read by [`crate::systems::persist_snapshot`]
#[derive(Event)]
pub struct DesyncEvent {
    pub frame: Frame,
}
