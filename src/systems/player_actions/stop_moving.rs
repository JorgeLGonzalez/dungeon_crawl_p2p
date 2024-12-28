use crate::{components::MoveThrottle, events::StopMovingEvent};
use bevy::prelude::{Commands, EventReader};

pub fn stop_moving(mut commands: Commands, mut event_reader: EventReader<StopMovingEvent>) {
    event_reader.read().for_each(|event| {
        commands.entity(event.player).remove::<MoveThrottle>();
    });
}
