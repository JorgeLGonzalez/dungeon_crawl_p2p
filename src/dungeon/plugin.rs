use super::{reveal_cheat, spawn_dungeon, zoom, DungeonEventsPlugin};
use crate::config::{game_mode, GameMode};
use crate::GameState;
use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

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
        let core_systems = (reveal_cheat, zoom)
            .in_set(DungeonCoreSet)
            .run_if(in_state(GameState::InGame));

        if game_mode(GameMode::SinglePlayer) {
            app.add_systems(
                Update,
                core_systems.run_if(|| game_mode(GameMode::SinglePlayer)),
            );
        } else {
            app.add_systems(GgrsSchedule, core_systems);
        }
    }
}
