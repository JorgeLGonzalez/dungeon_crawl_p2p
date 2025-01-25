use super::*;
use crate::{items::MagicItem, prelude::*};

pub type InventoryLabelQuery<'w, 's, 't> =
    Query<'w, 's, (Entity, &'t mut Text), (With<InventoryItem>, Without<InventoryTitle>)>;

pub struct HudInventorySync {
    items: Vec<MagicItem>,
    item_font: TextFont,
}

impl HudInventorySync {
    pub fn new(items: &[MagicItem], font_assets: &FontAssets) -> Self {
        Self {
            items: items.to_vec(),
            item_font: Self::create_item_font(font_assets),
        }
    }

    pub fn inventory_title(&self) -> String {
        format!("Inventory ({})", self.items.len())
    }

    pub fn remove_excess_ui_items(
        self,
        labels: &mut InventoryLabelQuery,
        commands: &mut Commands,
    ) -> Self {
        labels.iter().skip(self.items.len()).for_each(|(label, _)| {
            commands.entity(label).despawn_recursive();
        });

        self
    }

    pub fn spawn_ui_items(&self, commands: &mut Commands, ui_item_count: usize) -> Vec<Entity> {
        let create_item_bundle = |(index, item): (usize, &MagicItem)| {
            InventoryItemBundle::new(&item.label(), ui_item_count + 1 + index, &self.item_font)
        };

        self.items
            .iter()
            .skip(ui_item_count)
            .enumerate()
            .map(create_item_bundle)
            .map(|bundle| commands.spawn(bundle).id())
            .collect::<Vec<_>>()
    }

    pub fn update_existing_ui_items(self, ui_labels: &mut InventoryLabelQuery) -> Self {
        ui_labels
            .iter_mut()
            .zip(self.items.iter().map(|i| i.label()))
            .enumerate()
            .map(|(index, ((_, text), label))| (text, index + 1, label))
            .for_each(|(mut text, index, label)| {
                text.0 = InventoryItemBundle::ui_label(index, &label);
            });

        self
    }

    fn create_item_font(font_assets: &FontAssets) -> TextFont {
        TextFont {
            font: font_assets.hud_font.clone(),
            font_size: 16.,
            ..default()
        }
    }
}
