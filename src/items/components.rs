use crate::{hud::TooltipLabel, prelude::*};

#[derive(Component, Clone, Copy, Hash)]
pub struct Grabbable;

#[derive(Component, Clone, Debug, Copy, Hash)]
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

    pub fn healing_amount(&self) -> HealthUnit {
        match self {
            MagicItem::HealingPotion => 6,
            MagicItem::HealingPotionWeak => 2,
        }
    }

    pub fn label(&self) -> String {
        let healing = self.healing_amount();
        match self {
            MagicItem::HealingPotion => format!("Healing Potion ({healing} hp)"),
            MagicItem::HealingPotionWeak => format!("Weak Healing Potion ({healing} hp)"),
        }
    }

    pub fn tooltip(&self) -> TooltipLabel {
        TooltipLabel(self.label())
    }
}
