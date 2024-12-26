use bevy::{
    math::Vec2,
    prelude::{Entity, Event},
};

#[derive(Event)]
pub struct PlayerAttackEvent {
    pub monster: Entity,
    pub player_id: usize,
    pub pos: Vec2,
}

impl PlayerAttackEvent {
    pub fn new(player_id: usize, pos: Vec2, monster: Entity) -> Self {
        Self {
            monster,
            player_id,
            pos,
        }
    }
}

#[derive(Event)]
pub struct PlayerMoveEvent {
    pub player: Entity,
    pub player_id: usize,
    pub pos: Vec2,
}

impl PlayerMoveEvent {
    pub fn new(player: Entity, player_id: usize, pos: Vec2) -> Self {
        Self {
            player,
            player_id,
            pos,
        }
    }
}

#[derive(Event)]
pub struct PlayerMoveIntentEvent {
    pub player: Entity,
    pub player_id: usize,
    pub direction: Vec2,
}

impl PlayerMoveIntentEvent {
    pub fn new(player: Entity, player_id: usize, direction: Vec2) -> Self {
        Self {
            direction,
            player,
            player_id,
        }
    }
}
