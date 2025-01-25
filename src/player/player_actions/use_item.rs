use super::{Inventory, InventoryUpdatedEvent, UseItemEvent};
use crate::{health::DrinkPotionEvent, prelude::*};

pub fn use_item(
    mut drink_potion_event: EventWriter<DrinkPotionEvent>,
    mut inventory_updated_event: EventWriter<InventoryUpdatedEvent>,
    mut players: Query<&mut Inventory, With<Player>>,
    mut use_item_event: EventReader<UseItemEvent>,
) {
    use_item_event.read().for_each(
        |UseItemEvent {
             player,
             player_id,
             item_index,
         }| {
            let mut inventory = players.get_mut(*player).expect("Player not found");
            if inventory.items.len() > *item_index as usize {
                let item = inventory.items.remove(*item_index as usize);
                drink_potion_event.send(DrinkPotionEvent::new(
                    *player,
                    *player_id,
                    item.healing_amount(),
                ));
                inventory_updated_event
                    .send(InventoryUpdatedEvent::new(inventory.clone(), *player_id));
                info!("Use item event: {:?}", item.label());
            }
        },
    );
}
