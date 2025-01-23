use bevy::prelude::*;
use bevy_ggrs::ggrs::Frame;

pub struct CommonEventsPlugin;

impl Plugin for CommonEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DesyncEvent>()
            .add_event::<SnapshotStateEvent>();
    }
}

/// Used when an out-of-sync is detected by GGRS. Dispatched by [`crate::systems::handle_ggrs_events`]
/// and read by [`crate::systems::persist_snapshot`]
#[derive(Event)]
pub struct DesyncEvent {
    pub frame: Frame,
}

#[derive(Event)]
pub struct SnapshotStateEvent {
    pub player_id: usize,
}

impl SnapshotStateEvent {
    pub fn new(player_id: usize) -> Self {
        Self { player_id }
    }
}
