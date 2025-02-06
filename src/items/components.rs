use crate::{hud::TooltipLabel, prelude::*};
use serde::Deserialize;

#[derive(Bundle)]
pub struct MagicItemBundle {
    pub item: MagicItem,
    pub grabbable: Grabbable,
    pub sprite: Sprite,
    pub tooltip_label: TooltipLabel,
    pub transform: Transform,
    pub visibility: Visibility,
}

impl MagicItemBundle {
    pub fn new(template: &MagicItemTemplate, pos: Vec2) -> Self {
        let item = template.item;

        Self {
            item,
            grabbable: Grabbable,
            sprite: Sprite {
                color: template.color(),
                custom_size: Some(Vec2::new(config::TILE_WIDTH, config::TILE_HEIGHT)),
                ..default()
            },
            tooltip_label: TooltipLabel(item.label()),
            transform: Transform::from_translation(pos.extend(config::ITEM_Z_LAYER)),
            visibility: Visibility::Hidden,
        }
    }
}

#[derive(Component, Clone, Copy, Hash)]
pub struct Grabbable;

#[derive(Debug, Deserialize)]
pub struct MagicItemTemplate {
    pub frequency: usize,
    pub item: MagicItem,
    color: Srgba,
}

impl MagicItemTemplate {
    pub fn color(&self) -> Color {
        self.color.into()
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
