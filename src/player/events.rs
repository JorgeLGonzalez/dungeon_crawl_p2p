use super::{Inventory, PlayerId};
use crate::health::DamageUnit;
use bevy::prelude::*;

pub struct PlayerEventsPlugin;

impl Plugin for PlayerEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GrabItemEvent>()
            .add_event::<PlayerAttacksEvent>()
            .add_event::<PlayerMovesEvent>()
            .add_event::<PlayerMoveIntentEvent>()
            .add_event::<StopMovingEvent>()
            .add_event::<InventoryUpdatedEvent>();
    }
}

#[derive(Event)]
pub struct GrabItemEvent {
    pub player: Entity,
    pub player_id: usize,
}

impl GrabItemEvent {
    pub fn new(player: Entity, player_id: usize) -> Self {
        Self { player, player_id }
    }
}

/// Event: Player attacks monster
#[derive(Event)]
pub struct PlayerAttacksEvent {
    pub damage: DamageUnit,
    pub monster: Entity,
    pub player_id: usize,
    pub pos: IVec2,
}

impl PlayerAttacksEvent {
    pub fn new(player_id: usize, pos: IVec2, monster: Entity, damage: DamageUnit) -> Self {
        Self {
            damage,
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
    pub pos: IVec2,
}

impl PlayerMovesEvent {
    pub fn new(player: Entity, player_id: usize, pos: IVec2) -> Self {
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
    pub direction: IVec2,
}

impl PlayerMoveIntentEvent {
    pub fn new(player: Entity, player_id: usize, direction: IVec2) -> Self {
        Self {
            direction,
            player,
            player_id,
        }
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

#[derive(Event)]
pub struct InventoryUpdatedEvent {
    pub inventory: Inventory,
    pub player_id: PlayerId,
}

impl InventoryUpdatedEvent {
    pub fn new(inventory: Inventory, player_id: PlayerId) -> Self {
        Self {
            inventory,
            player_id,
        }
    }
}
