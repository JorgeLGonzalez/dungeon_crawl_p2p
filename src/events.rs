use bevy::{
    math::Vec2,
    prelude::{Entity, Event},
};

/// Event: Monster attacks player
#[derive(Event)]
pub struct MonsterAttacksEvent {
    pub monster: Entity,
    pub player: Entity,
    pub player_id: usize,
    pub pos: Vec2,
}

impl MonsterAttacksEvent {
    pub fn new(monster: Entity, player: Entity, player_id: usize, pos: Vec2) -> Self {
        Self {
            monster,
            player,
            player_id,
            pos,
        }
    }
}

/// Event: Player attacks monster
#[derive(Event)]
pub struct PlayerAttacksEvent {
    pub monster: Entity,
    pub player_id: usize,
    pub pos: Vec2,
}

impl PlayerAttacksEvent {
    pub fn new(player_id: usize, pos: Vec2, monster: Entity) -> Self {
        Self {
            monster,
            player_id,
            pos,
        }
    }
}

#[derive(Event)]
pub struct PlayerMovesEvent {
    pub player: Entity,
    pub player_id: usize,
    pub pos: Vec2,
}

impl PlayerMovesEvent {
    pub fn new(player: Entity, player_id: usize, pos: Vec2) -> Self {
        Self {
            player,
            player_id,
            pos,
        }
    }
}

#[derive(Event, Clone, Copy)]
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

#[derive(Event)]
pub struct SnapshotStateEvent {
    pub player_id: usize,
}

impl SnapshotStateEvent {
    pub fn new(player_id: usize) -> Self {
        Self { player_id }
    }
}

#[derive(Event)]
pub struct StopMovingEvent {
    pub player: Entity,
}

impl StopMovingEvent {
    pub fn new(player: Entity) -> Self {
        Self { player }
    }
}
