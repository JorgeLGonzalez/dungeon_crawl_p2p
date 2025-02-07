use super::config;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct InventoryItemBundle {
    pub interaction: Interaction,
    pub inventory_item: InventoryItem,
    pub text: Text,
    pub text_font: TextFont,
    pub z_index: GlobalZIndex,
}

impl InventoryItemBundle {
    pub fn new(label: &str, index: usize, font: &TextFont) -> Self {
        Self {
            interaction: Interaction::None,
            inventory_item: InventoryItem,
            text: Text(Self::ui_label(index, label)),
            text_font: font.clone(),
            z_index: GlobalZIndex(config::Z_INDEX),
        }
    }

    pub fn ui_label(index: usize, item_label: &str) -> String {
        format!("{index}: {item_label}")
    }

    pub fn index_from_text(item_text: &Text) -> u8 {
        item_text
            .0
            .split(":")
            .next()
            .and_then(|s| s.parse::<u8>().ok())
            .map(|idx| idx - 1)
            .unwrap_or_else(|| {
                panic!("Malformed inventory item text '{}'", item_text.0);
            })
    }
}

#[derive(Component)]
pub struct InventoryItem;
#[derive(Component)]
pub struct InventoryPanel;

#[derive(Component)]
pub struct InventoryTitle;
