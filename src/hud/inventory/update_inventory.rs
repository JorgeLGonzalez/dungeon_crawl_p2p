use super::*;
use crate::{
    player::{InventoryUpdatedEvent, LocalPlayer},
    prelude::*,
};
use bevy_ggrs::LocalPlayers;

pub fn update_inventory(
    mut commands: Commands,
    mut events: EventReader<InventoryUpdatedEvent>,
    mut inventory_title: Query<&mut Text, With<InventoryTitle>>,
    mut labels: InventoryLabelQuery,
    font_assets: Res<FontAssets>,
    inventory_panel: Query<Entity, With<InventoryPanel>>,
    local_players: Res<LocalPlayers>,
) {
    events
        .read()
        .filter(|e| LocalPlayer::is_local_player_id(e.player_id, &local_players))
        .for_each(|event| {
            let sync = HudInventorySync::new(&event.inventory.items, &font_assets)
                .update_existing_ui_items(&mut labels)
                .remove_excess_ui_items(&mut labels, &mut commands);

            let ui_items_to_append = sync.spawn_ui_items(&mut commands, labels.iter().count());
            commands
                .entity(inventory_panel.single())
                .add_children(&ui_items_to_append);

            inventory_title.single_mut().0 = sync.inventory_title();
        });
}
