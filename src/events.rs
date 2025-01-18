use bevy::prelude::Event;
use bevy::prelude::*;

/**
 * REMEMBER to use bevy App `add_events` to register events!
 */

#[derive(Event)]
pub struct RecalculateFovEvent {
    pub entity: Entity,
    pub pos: IVec2,
}

impl RecalculateFovEvent {
    pub fn new(entity: Entity, pos: IVec2) -> Self {
        Self { entity, pos }
    }
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
