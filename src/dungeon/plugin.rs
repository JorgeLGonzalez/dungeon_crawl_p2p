use super::{reveal_map, spawn_dungeon, zoom, DungeonEventsPlugin};
use crate::{common, prelude::*};

#[derive(SystemSet, Clone, Debug, Eq, Hash, PartialEq)]
pub struct SpawnDungeonSet;

#[derive(SystemSet, Clone, Debug, Eq, Hash, PartialEq)]
pub struct DungeonCoreSet;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DungeonEventsPlugin).add_systems(
            OnEnter(GameState::InGame),
            spawn_dungeon.in_set(SpawnDungeonSet),
        );

        common::add_core_systems(app, (reveal_map, zoom).in_set(DungeonCoreSet));
    }
}
