use bevy::prelude::*;

#[derive(Bundle)]
pub struct InventoryItemBundle {
    pub inventory_item: InventoryItem,
    pub text: Text,
    pub text_font: TextFont,
}

impl InventoryItemBundle {
    pub fn new(label: &str, index: usize, font: &TextFont) -> Self {
        Self {
            inventory_item: InventoryItem,
            text: Text(Self::ui_label(index, label)),
            text_font: font.clone(),
        }
    }

    pub fn ui_label(index: usize, item_label: &str) -> String {
        format!("{index}: {item_label}")
    }
}

#[derive(Component)]
pub struct InventoryItem;
#[derive(Component)]
pub struct InventoryPanel;

#[derive(Component)]
pub struct InventoryTitle;
