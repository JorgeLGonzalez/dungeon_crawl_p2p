use crate::{player::InventoryUpdatedEvent, prelude::*};

pub fn update_inventory(
    mut events: EventReader<InventoryUpdatedEvent>,
    // local_players: Res<LocalPlayers>,
) {
    for event in events.read() {
        info!(
            "Player {} Added to inventory now with {} items",
            event.player_id,
            event.inventory.items.len()
        );

        //   if LocalPlayer::is_local(player, &local_players) {
        //     update_inventory_event.send(UpdateInventoryEvent::new(*item));
        // }
    }
}
