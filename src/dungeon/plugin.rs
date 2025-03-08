use super::*;
use crate::{common, prelude::*};

#[derive(SystemSet, Clone, Debug, Eq, Hash, PartialEq)]
pub struct SpawnDungeonSet;

#[derive(SystemSet, Clone, Debug, Eq, Hash, PartialEq)]
pub struct DungeonCoreSet;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DungeonEventsPlugin).add_systems(
            OnEnter(GameState::DungeonSpawning),
            (despawn_dungeon, spawn_dungeon)
                .in_set(SpawnDungeonSet)
                .chain(),
        );

        common::add_core_systems(app, (reveal_map, zoom).in_set(DungeonCoreSet));
    }
}
