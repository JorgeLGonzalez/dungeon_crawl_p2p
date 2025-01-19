use crate::{hud::TooltipLabel, prelude::*};

#[derive(Component)]
pub struct Grabbable;

#[derive(Component, Clone, Copy)]
pub enum MagicItem {
    HealingPotion,
    HealingPotionWeak,
}

impl MagicItem {
    pub fn color(&self) -> Color {
        match self {
            MagicItem::HealingPotion => Color::srgb(0., 0., 1.),
            MagicItem::HealingPotionWeak => Color::srgb(0.5, 0.5, 0.9),
        }
    }

    pub fn healing_amount(&self) -> u8 {
        match self {
            MagicItem::HealingPotion => 10,
            MagicItem::HealingPotionWeak => 2,
        }
    }

    pub fn tooltip(&self) -> TooltipLabel {
        match self {
            MagicItem::HealingPotion => {
                TooltipLabel(format!("Healing Potion: {} hp", self.healing_amount()))
            }
            MagicItem::HealingPotionWeak => {
                TooltipLabel(format!("Weak Healing Potion: {} hp", self.healing_amount()))
            }
        }
    }
}
