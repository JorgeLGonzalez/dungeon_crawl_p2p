use super::*;
use crate::{common, prelude::*};
use bevy_ggrs::{ggrs::Frame, GgrsApp, GgrsSchedule, RollbackFrameCount};

#[derive(SystemSet, Clone, Debug, Eq, Hash, PartialEq)]
pub struct SpawnDungeonSet;

#[derive(SystemSet, Clone, Debug, Eq, Hash, PartialEq)]
pub struct DungeonCoreSet;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DungeonEventsPlugin);

        app.add_systems(
            GgrsSchedule,
            (check_respawn_state, despawn_dungeon, spawn_dungeon)
                .in_set(SpawnDungeonSet)
                .run_if(in_state(GameState::DungeonSpawning))
                .chain(),
        );

        common::add_core_systems(app, (reveal_map, zoom).in_set(DungeonCoreSet));

        app.insert_resource(RespawnState::None)
            .rollback_resource_with_copy::<RespawnState>();
    }
}

#[derive(Resource, Copy, Clone, Debug)]
pub enum RespawnState {
    Complete(Frame),
    Init(Frame),
    None,
    Pending(Frame),
}

fn check_respawn_state(
    frame: Res<RollbackFrameCount>,
    respawn: Res<RespawnState>,
    next_state: Res<NextState<GameState>>,
) {
    if let NextState::Pending(pending_state) = *next_state {
        panic!("Attempt to run DungeonSpawn with a pending state transition to {pending_state:?}");
    };

    match *respawn {
        RespawnState::None => {}
        RespawnState::Pending(_) => {}
        _ => {
            panic!("Attempt to run DungeonSpawn with a RespawnState {respawn:?}");
        }
    }

    // could check RespawnState::Complete frame to make sure its not too recent

    let frame = frame.0;
    info!("|HIGHLIGHT| Checks pass for Dungeon spawning (frame {frame})");
}
