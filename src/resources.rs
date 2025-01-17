pub mod assets;
pub mod config;
mod random_generator;

use bevy::prelude::Event;
use bevy_ggrs::ggrs::Frame;
pub use random_generator::{RandomCounter, RandomGenerator};

/// Used when an out-of-sync is detected by GGRS. Dispatched by [`crate::systems::handle_ggrs_events`]
/// and read by [`crate::systems::persist_snapshot`]
#[derive(Event)]
pub struct DesyncEvent {
    pub frame: Frame,
}
