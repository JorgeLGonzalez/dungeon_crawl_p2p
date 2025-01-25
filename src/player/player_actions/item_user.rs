use super::{Inventory, InventoryUpdatedEvent, UseItemEvent};
use crate::{health::DrinkPotionEvent, player::PlayerId, prelude::*};

pub type InventoryUsageQuery<'w, 's, 'i> = Query<'w, 's, &'i mut Inventory, With<Player>>;

pub struct ItemUser<'a> {
    player: Entity,
    player_id: PlayerId,
    inventory: Mut<'a, Inventory>,
    item_index: usize,
}

impl<'a> ItemUser<'a> {
    pub fn try_new(event: &UseItemEvent, players: &'a mut InventoryUsageQuery) -> Option<Self> {
        let inventory = players.get_mut(event.player).expect("Player not found");

        if inventory.items.len() <= event.item_index as usize {
            None
        } else {
            Some(Self {
                player: event.player,
                player_id: event.player_id,
                inventory,
                item_index: event.item_index as usize,
            })
        }
    }

    pub fn create_inventory_updated_event(&self) -> InventoryUpdatedEvent {
        InventoryUpdatedEvent::new(self.inventory.clone(), self.player_id)
    }

    pub fn use_item(&mut self) -> DrinkPotionEvent {
        let item = self.inventory.items.remove(self.item_index);
        info!("Use item event: {:?}", item.label());

        DrinkPotionEvent::new(self.player, self.player_id, item.healing_amount())
    }
}
