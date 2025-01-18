use bevy::prelude::*;

pub struct FovEventsPlugin;

impl Plugin for FovEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RecalculateFovEvent>();
    }
}

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
