use bevy::prelude::*;

#[derive(Debug)]
pub struct TooltipDisplayInfo {
    pub game_pos: Vec2,
    pub target_entity: Entity,
    pub text: String,
}

impl TooltipDisplayInfo {
    pub fn new(game_pos: Vec2, target_entity: Entity, text: String) -> Self {
        Self {
            game_pos,
            target_entity,
            text,
        }
    }
}
