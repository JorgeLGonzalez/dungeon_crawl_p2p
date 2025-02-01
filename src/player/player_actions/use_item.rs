use super::{InventoryUpdatedEvent, InventoryUsageQuery, ItemUser, UseItemEvent};
use crate::{health::DrinkPotionEvent, prelude::*};

pub fn use_item(
    mut drink_potion_event: EventWriter<DrinkPotionEvent>,
    mut inventory_updated_event: EventWriter<InventoryUpdatedEvent>,
    mut players: InventoryUsageQuery,
    mut use_item_event: EventReader<UseItemEvent>,
) {
    use_item_event.read().for_each(|event| {
        let Some(mut item_user) = ItemUser::try_new(&event, &mut players) else {
            return;
        };

        let drink_potion = item_user.use_item();
        drink_potion_event.send(drink_potion);

        inventory_updated_event.send(item_user.create_inventory_updated_event());
    });
}
