use super::{events::*, monster_actions::MonsterMoveTracker, spawn_monsters};
use crate::{dungeon, GameState};
use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MonstersCoreSet;

pub struct MonstersPlugin;

impl Plugin for MonstersPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MonsterActedEvent>()
            .add_event::<MonsterAttacksEvent>()
            .add_event::<MonsterMovesEvent>()
            .init_resource::<MonsterMoveTracker>()
            .add_systems(
                OnEnter(GameState::InGame),
                spawn_monsters.after(dungeon::SpawnDungeonSet),
            );
        // Add your monster-specific systems here
    }
}
