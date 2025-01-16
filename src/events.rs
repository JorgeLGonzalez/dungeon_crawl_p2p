use crate::components::DamageUnit;
use bevy::prelude::Event;
use bevy::prelude::*;

/**
 * REMEMBER to use bevy App `add_events` to register events!
 */

#[derive(Event)]
pub struct MonsterActedEvent {
    pub monster: Entity,
}

impl MonsterActedEvent {
    pub fn new(monster: Entity) -> Self {
        Self { monster }
    }
}

/// Event: Monster attacks player
#[derive(Event)]
pub struct MonsterAttacksEvent {
    pub damage: DamageUnit,
    pub monster: Entity,
    pub player: Entity,
    pub player_id: usize,
    pub pos: IVec2,
}

impl MonsterAttacksEvent {
    pub fn new(
        monster: Entity,
        damage: DamageUnit,
        player: Entity,
        player_id: usize,
        pos: IVec2,
    ) -> Self {
        Self {
            damage,
            monster,
            player,
            player_id,
            pos,
        }
    }
}

#[derive(Event)]
pub struct MonsterMovesEvent {
    pub monster: Entity,
    pub movement: IVec2,
    pub pos: IVec2,
    pub rng_counter: u128,
}

impl MonsterMovesEvent {
    pub fn new(monster: Entity, movement: IVec2, pos: IVec2, rng_counter: u128) -> Self {
        Self {
            monster,
            movement,
            pos,
            rng_counter,
        }
    }
}


#[derive(Event)]
pub struct RecalculateFovEvent {
    pub entity: Entity,
    pub pos: IVec2,
}

impl RecalculateFovEvent {
    pub fn new(entity: Entity, pos: IVec2) -> Self {
        Self { entity, pos }
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
