use super::{
    item_grabber::EquipEvent, GrabItemEvent, ItemGrabber, ItemQuery, PlayerInventoryQuery,
};
use crate::{
    player::{InventoryUpdatedEvent, WeaponWieldedEvent},
    prelude::*,
};

/// Handle a GrabItemEvent by grabbing the item the player is over (if any),
/// despawning it, and then inserting it into the player's inventory.
/// Sends an InventoryUpdatedEvent to update the HUD.
pub fn grab_item(
    mut commands: Commands,
    mut grab_events: EventReader<GrabItemEvent>,
    mut inventory_event: EventWriter<InventoryUpdatedEvent>,
    mut players: PlayerInventoryQuery,
    mut wield_event: EventWriter<WeaponWieldedEvent>,
    items: ItemQuery,
) {
    grab_events
        .read()
        .map(ItemGrabber::new)
        .filter_map(|grabber| grabber.find_item_under_player(&items, &players))
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|grabber| {
            let event = grabber.grab_item(&mut commands, &mut players);
            match event {
                EquipEvent::InventoryUpdate(event) => {
                    inventory_event.send(event);
                }
                EquipEvent::Wield(event) => {
                    info!("Wielding weapon");
                    wield_event.send(event);
                }
            }
        });
}
