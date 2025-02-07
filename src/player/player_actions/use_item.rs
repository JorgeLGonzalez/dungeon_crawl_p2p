use super::*;
use crate::{dungeon::RevealDungeonCheatEvent, health::DrinkPotionEvent, prelude::*};

pub fn use_item(
    mut drink_potion_event: EventWriter<DrinkPotionEvent>,
    mut inventory_updated_event: EventWriter<InventoryUpdatedEvent>,
    mut players: InventoryUsageQuery,
    mut reveal_map_event: EventWriter<RevealDungeonCheatEvent>,
    mut use_item_event: EventReader<UseItemEvent>,
) {
    use_item_event.read().for_each(|event| {
        let Some(mut item_user) = ItemUser::try_new(&event, &mut players) else {
            return;
        };

        match item_user.use_item() {
            ItemUseEvent::DrinkPotion(event) => {
                drink_potion_event.send(event);
            }
            ItemUseEvent::RevealMap(event) => {
                reveal_map_event.send(event);
            }
        }

        inventory_updated_event.send(item_user.create_inventory_updated_event());
    });
}
