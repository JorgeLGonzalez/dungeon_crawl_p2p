use crate::prelude::*;
use serde::Deserialize;

#[derive(Component, Clone, Copy, Hash)]
pub struct Grabbable;

#[derive(Debug, Deserialize)]
pub struct MagicItemTemplate {
    pub frequency: usize,
    color: Srgba,
    item_type: MagicItem,
}

impl MagicItemTemplate {
    pub fn color(&self) -> Color {
        self.color.into()
    }

    pub fn to_magic_item(&self) -> MagicItem {
        self.item_type
    }
}

#[derive(Component, Clone, Debug, Deserialize, Copy, Hash)]
pub enum MagicItem {
    HealingPotion(HealthUnit),
}

impl MagicItem {
    pub fn healing_amount(&self) -> HealthUnit {
        match self {
            MagicItem::HealingPotion(amount) => *amount,
        }
    }

    pub fn label(&self) -> String {
        match self {
            MagicItem::HealingPotion(hp) => format!("Healing Potion ({hp} hp)"),
        }
    }
}
