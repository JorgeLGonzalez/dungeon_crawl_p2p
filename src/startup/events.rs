use bevy::prelude::Event;
use bevy_ggrs::ggrs::Frame;

/// Used when an out-of-sync is detected by GGRS. Dispatched by [`crate::systems::handle_ggrs_events`]
/// and read by [`crate::systems::persist_snapshot`]
#[derive(Event)]
pub struct DesyncEvent {
    pub frame: Frame,
}
