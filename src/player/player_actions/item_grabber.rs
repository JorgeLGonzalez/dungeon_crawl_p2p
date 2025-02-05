use super::{GrabItemEvent, Inventory};
use crate::{
    items::{Grabbable, MagicItem, Weapon},
    player::{InventoryUpdatedEvent, PlayerId, WeaponWieldedEvent},
    prelude::*,
};

pub(super) type ItemQuery<'w, 's, 'i, 't> =
    Query<'w, 's, (Entity, &'i MagicItem, &'t Transform), With<Grabbable>>;
pub(super) type PlayerInventoryQuery<'w, 's, 'i, 't, 'p> =
    Query<'w, 's, (&'i mut Inventory, &'t Transform, Option<&'p Weapon>), With<Player>>;

pub(super) struct ItemGrabber {
    item: Option<MagicItem>,
    item_entity: Option<Entity>,
    player: Entity,
    player_id: PlayerId,
}

impl ItemGrabber {
    pub fn new(event: &GrabItemEvent) -> Self {
        Self {
            item: None,
            item_entity: None,
            player: event.player,
            player_id: event.player_id,
        }
    }

    pub fn find_item_under_player(
        self,
        items: &ItemQuery,
        players: &PlayerInventoryQuery,
    ) -> Option<Self> {
        let player_pos = players
            .get(self.player)
            .map(|(_, t, _)| t.translation.truncate().as_ivec2())
            .expect("Player not found");

        items
            .iter()
            .find(|(.., t)| t.translation.truncate().as_ivec2() == player_pos)
            .map(|(item_entity, item, ..)| Self {
                item: Some(item.clone()),
                item_entity: Some(item_entity),
                ..self
            })
    }

    pub fn grab_item(
        &self,
        commands: &mut Commands,
        players: &mut PlayerInventoryQuery,
    ) -> EquipEvent {
        let item_entity = self.item_entity.unwrap();
        let item = self.item.unwrap();

        info!("Player {} grabs item {item_entity}", self.player_id);
        commands.entity(item_entity).despawn_recursive();

        match item {
            MagicItem::HealingPotion(_) => {
                let mut inventory = players
                    .get_mut(self.player)
                    .map(|(inventory, ..)| inventory)
                    .expect("Player not found");

                inventory.items.push(item);

                EquipEvent::InventoryUpdate(InventoryUpdatedEvent::new(
                    inventory.clone(),
                    self.player_id,
                ))
            }
            MagicItem::Weapon(weapon) => {
                commands.entity(self.player).insert(weapon);

                EquipEvent::Wield(WeaponWieldedEvent::new(self.player_id, item))
            }
        }
    }
}

pub enum EquipEvent {
    InventoryUpdate(InventoryUpdatedEvent),
    Wield(WeaponWieldedEvent),
}
