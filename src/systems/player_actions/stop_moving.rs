use crate::{components::PlayerMovement, StopMovingEvent};
use bevy::{
    log::info,
    prelude::{EventReader, Query},
};

pub fn stop_moving(
    mut event_reader: EventReader<StopMovingEvent>,
    mut players: Query<&mut PlayerMovement>,
) {
    event_reader.read().for_each(|event| {
        if let Ok(mut movement) = players.get_mut(event.player) {
            movement.direction = None;
        }
    });
}
