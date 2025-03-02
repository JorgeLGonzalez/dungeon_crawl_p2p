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
    pub fn new(item: MagicItem, pos: Vec2) -> Self {
        Self {
            item,
            grabbable: Grabbable,
            sprite: Sprite {
                color: item.color(),
                custom_size: Some(Vec2::new(TILE_WIDTH, TILE_HEIGHT)),
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
}

#[derive(Component, Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
pub enum MagicItem {
    HealingPotion(HealthUnit),
    Map,
    Weapon(Weapon),
}

impl MagicItem {
    pub fn color(&self) -> Color {
        match self {
            MagicItem::HealingPotion(h) => match h {
                2 => Color::srgb(0.5, 0.5, 0.9),
                6 => Color::srgb(0., 0., 1.),
                _ => unreachable!(),
            },
            MagicItem::Map => Color::srgb(0.8, 0.7, 1.0),
            MagicItem::Weapon(w) => w.color(),
        }
    }

    pub fn healing_amount(&self) -> HealthUnit {
        match self {
            MagicItem::HealingPotion(amount) => *amount,
            _ => 0,
        }
    }

    pub fn label(&self) -> String {
        match self {
            MagicItem::HealingPotion(hp) => format!("Healing Potion ({hp} hp)"),
            MagicItem::Map => "Magic Map".to_string(),
            MagicItem::Weapon(w) => format!("{:?} Sword ({} hp)", w.sword, w.damage),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Hash)]
pub enum Sword {
    Huge,
    Rusty,
    Shiny,
}

impl Sword {
    pub fn color(&self) -> Color {
        match self {
            Sword::Huge => Color::srgb(1.0, 0.8, 0.),
            Sword::Rusty => Color::srgb(0.8, 0.6, 0.),
            Sword::Shiny => Color::srgb(0.8, 0.8, 0.),
        }
    }
}

#[derive(Component, Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct Weapon {
    pub damage: HealthUnit,
    pub sword: Sword,
}

impl Weapon {
    pub fn color(&self) -> Color {
        self.sword.color()
    }
}
