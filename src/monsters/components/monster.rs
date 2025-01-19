use crate::{hud::TooltipLabel, prelude::*};

#[derive(Component, Clone, Copy, Debug)]
pub enum Monster {
    Ettin,
    Goblin,
    Ogre,
    Orc,
}

impl Monster {
    pub fn color(&self) -> Color {
        match self {
            Monster::Ettin => Color::srgb(0.9, 0.1, 0.1),
            Monster::Ogre => Color::srgb(0.8, 0.2, 0.2),
            Monster::Orc => Color::srgb(0.7, 0.3, 0.3),
            Monster::Goblin => Color::srgb(0.6, 0.4, 0.4),
        }
    }

    pub fn damage(&self) -> Damage {
        match self {
            Monster::Ettin => Damage(3),
            Monster::Goblin => Damage(1),
            Monster::Ogre => Damage(2),
            Monster::Orc => Damage(1),
        }
    }

    pub fn health(&self) -> Health {
        match self {
            Monster::Ettin => Health::new(10),
            Monster::Goblin => Health::new(1),
            Monster::Ogre => Health::new(2),
            Monster::Orc => Health::new(2),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Monster::Ettin => "Ettin",
            Monster::Goblin => "Goblin",
            Monster::Ogre => "Ogre",
            Monster::Orc => "Orc",
        }
    }

    pub fn tooltip(&self) -> TooltipLabel {
        let (name, health) = (self.name(), self.health().max);

        TooltipLabel(format!("{name}: {health} hp"))
    }
}
