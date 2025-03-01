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
    pub fn new(template: &MonsterTemplate, pos: Vec2) -> Self {
        let monster = template.monster;

        Self {
            monster,
            damage: Damage(template.damage),
            fov: FieldOfView::new(config::MONSTER_FOV_RADIUS),
            health: Health::new(template.health),
            last_action: LastAction::new(),
            obstacle: Obstacle::Monster,
            sprite: Sprite {
                color: template.color(),
                custom_size: Some(Vec2::new(TILE_WIDTH, TILE_HEIGHT)),
                ..default()
            },
            tooltip_label: TooltipLabel(template.label()),
            transform: Transform::from_translation(pos.extend(config::MONSTER_Z_LAYER)),
            visibility: Visibility::Hidden,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MonsterTemplate {
    pub damage: DamageUnit,
    pub frequency: usize,
    pub health: HealthUnit,
    pub monster: Monster,
    color: Srgba,
}

impl MonsterTemplate {
    pub fn color(&self) -> Color {
        self.color.into()
    }

    pub fn label(&self) -> String {
        let (name, health) = (self.monster.name(), self.health);

        format!("{name}: {health} hp")
    }
}

#[derive(Component, Clone, Copy, Debug, Deserialize, Hash)]
pub enum Monster {
    Ettin,
    Goblin,
    Ogre,
    Orc,
}

impl Monster {
    pub fn name(&self) -> &str {
        match self {
            Monster::Ettin => "Ettin",
            Monster::Goblin => "Goblin",
            Monster::Ogre => "Ogre",
            Monster::Orc => "Orc",
        }
    }
}
