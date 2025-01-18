use crate::health::DamageUnit;
use bevy::prelude::*;

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
