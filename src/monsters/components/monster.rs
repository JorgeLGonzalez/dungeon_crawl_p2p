use super::LastAction;
use crate::{hud::TooltipLabel, player::Obstacle, prelude::*};
use serde::Deserialize;

#[derive(Bundle)]
pub struct MonsterBundle {
    pub monster: Monster,
    pub damage: Damage,
    pub fov: FieldOfView,
    pub health: Health,
    pub last_action: LastAction,
    pub obstacle: Obstacle,
    pub sprite: Sprite,
    pub tooltip_label: TooltipLabel,
    pub transform: Transform,
    pub visibility: Visibility,
}

impl MonsterBundle {
    pub fn new(monster: Monster, pos: Vec2) -> Self {
        Self {
            monster,
            damage: Damage(monster.damage()),
            fov: FieldOfView::new(config::MONSTER_FOV_RADIUS),
            health: Health::new(monster.health()),
            last_action: LastAction::new(),
            obstacle: Obstacle::Monster,
            sprite: Sprite {
                color: monster.color(),
                custom_size: Some(Vec2::new(TILE_WIDTH, TILE_HEIGHT)),
                ..default()
            },
            tooltip_label: TooltipLabel(monster.label()),
            transform: Transform::from_translation(pos.extend(config::MONSTER_Z_LAYER)),
            visibility: Visibility::Hidden,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MonsterTemplate {
    pub frequency: usize,
    pub monster: Monster,
}

#[derive(Component, Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
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
            Monster::Goblin => Color::srgb(0.6, 0.4, 0.4),
            Monster::Ogre => Color::srgb(0.8, 0.2, 0.2),
            Monster::Orc => Color::srgb(0.7, 0.3, 0.3),
        }
    }

    pub fn damage(&self) -> DamageUnit {
        match self {
            Monster::Ettin => 3,
            Monster::Goblin => 1,
            Monster::Ogre => 2,
            Monster::Orc => 1,
        }
    }

    pub fn health(&self) -> HealthUnit {
        match self {
            Monster::Ettin => 10,
            Monster::Goblin => 1,
            Monster::Ogre => 2,
            Monster::Orc => 2,
        }
    }

    pub fn label(&self) -> String {
        let (name, health) = (self.name(), self.health());

        format!("{name}: {health} hp")
    }

    pub fn name(&self) -> &str {
        match self {
            Monster::Ettin => "Ettin",
            Monster::Goblin => "Goblin",
            Monster::Ogre => "Ogre",
            Monster::Orc => "Orc",
        }
    }
}
