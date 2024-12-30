use crate::events::RecalculateFovEvent;
use bevy::prelude::*;

pub fn recalculate_fov(mut recalculate_events: EventReader<RecalculateFovEvent>) {
    for event in recalculate_events.read() {
        info!("Recalculating FOV for {} at {}", event.entity, event.pos);
    }
}
