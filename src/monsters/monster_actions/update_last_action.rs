use super::MonsterActedEvent;
use crate::components::{LastAction, Monster};
use bevy::prelude::*;

pub fn update_last_action(
    mut acted_events: EventReader<MonsterActedEvent>,
    mut last_actions: Query<&mut LastAction, With<Monster>>,
    time: Res<Time>,
) {
    for event in acted_events.read() {
        last_actions
            .get_mut(event.monster)
            .expect("Inconceivable!")
            .time = time.elapsed_secs();
    }
}
