use super::{GrabItemEvent, Inventory};
use crate::{
    items::{Grabbable, MagicItem},
    player::InventoryUpdatedEvent,
    prelude::*,
};

/// Handle a GrabItemEvent by grabbing the item the player is over (if any),
/// despawning it, and then inserting it into the player's inventory.
pub fn grab_item(
    mut commands: Commands,
    mut grab_events: EventReader<GrabItemEvent>,
    mut inventory_event: EventWriter<InventoryUpdatedEvent>,
    mut players: Query<(&mut Inventory, &Transform), With<Player>>,
    items: Query<(Entity, &MagicItem, &Transform), With<Grabbable>>,
) {
    for event in grab_events.read() {
        let (mut inventory, player_pos) = players
            .get_mut(event.player)
            .map(|(i, t)| (i, t.translation.truncate().as_ivec2()))
            .expect("Player not found");

        let Some((item_entity, item)) = items
            .iter()
            .find(|(.., t)| t.translation.truncate().as_ivec2() == player_pos)
            .map(|(e, item, ..)| (e, item))
        else {
            continue;
        };

        info!("Player {} grabs item {item_entity:?}", event.player_id);
        commands.entity(item_entity).despawn_recursive();
        inventory.items.push(*item);
        inventory_event.send(InventoryUpdatedEvent::new(
            inventory.clone(),
            event.player_id,
        ));
    }
}
