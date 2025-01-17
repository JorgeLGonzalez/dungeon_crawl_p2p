use super::super::events::StopMovingEvent;
use crate::components::MoveThrottle;
use bevy::prelude::{Commands, EventReader};

pub fn stop_moving(mut commands: Commands, mut event_reader: EventReader<StopMovingEvent>) {
    event_reader.read().for_each(|event| {
        commands.entity(event.player).remove::<MoveThrottle>();
    });
}
