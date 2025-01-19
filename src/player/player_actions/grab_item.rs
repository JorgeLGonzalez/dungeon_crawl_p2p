use super::GrabItemEvent;
use crate::prelude::*;

pub fn grab_item(mut grab_event: EventReader<GrabItemEvent>) {
    for event in grab_event.read() {
        info!("Player {} grabs item", event.player_id);
    }
}
