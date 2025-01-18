use bevy::prelude::*;

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
