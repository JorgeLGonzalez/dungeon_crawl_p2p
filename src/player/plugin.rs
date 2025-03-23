use super::{
    camera::*, components::*, events::*, player_actions::*, spawn_players, teleport_players,
};
use crate::{
    common,
    config::{game_mode, GameMode},
    dungeon::{DungeonCoreSet, DungeonMap, RespawnState, SpawnDungeonSet},
    monsters::MonstersCoreSet,
    GameState,
};
use bevy::prelude::*;
use bevy_ggrs::{AdvanceWorldSet, GgrsApp, GgrsSchedule, ReadInputs, SaveWorldSet};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PlayerCoreSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SpawnPlayersSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, exit2);

        app.add_systems(
            GgrsSchedule,
            (
                teleport_players.run_if(resource_exists::<DungeonMap>),
                spawn_players,
                setup_camera,
            )
                .in_set(SpawnPlayersSet)
                .run_if(in_state(GameState::DungeonSpawning))
                .chain()
                .ambiguous_with(PlayerCoreSet)
                .after(SpawnDungeonSet),
        );

        let core_systems = (
            do_player_action,
            tick_move_throttle,
            stop_moving.run_if(on_event::<StopMovingEvent>),
            handle_move_intent.run_if(on_event::<PlayerMoveIntentEvent>),
            grab_item.run_if(on_event::<GrabItemEvent>),
            use_item.run_if(on_event::<UseItemEvent>),
            attack_monster.run_if(on_event::<PlayerAttacksEvent>),
            move_player.run_if(on_event::<PlayerMovesEvent>),
            follow_with_camera.after(move_player),
            exit_level.run_if(on_event::<ExitLevelEvent>),
        )
            .in_set(PlayerCoreSet)
            .chain()
            .ambiguous_with(DungeonCoreSet)
            // .ambiguous_with(SpawnItemsSet)
            .before(MonstersCoreSet);

        common::add_core_systems(app, core_systems);

        app.add_systems(
            GgrsSchedule,
            exit2
                .run_if(in_state(GameState::InGame))
                .before(SpawnDungeonSet)
                .before(AdvanceWorldSet::First)
                .after(SaveWorldSet::Snapshot)
                .ambiguous_with_all(),
        );

        if !game_mode(GameMode::SinglePlayer) {
            app.rollback_component_with_clone::<Inventory>()
                .checksum_component_with_hash::<Inventory>()
                .rollback_component_with_clone::<MoveThrottle>()
                .checksum_component_with_hash::<MoveThrottle>()
                .rollback_component_with_copy::<Player>()
                .checksum_component_with_hash::<Player>()
                .rollback_component_with_copy::<Obstacle>()
                .checksum_component_with_hash::<Obstacle>();

            app.add_systems(ReadInputs, read_player_inputs);
        }

        app.add_plugins(PlayerEventsPlugin);
    }
}

fn exit2(world: &mut World) {
    let pending_state = matches!(
        world.resource::<NextState<GameState>>(),
        NextState::Pending(GameState::DungeonSpawning)
    );
    let pending_respawn = matches!(world.resource::<RespawnState>(), RespawnState::Pending(..));

    if !pending_state && !pending_respawn {
        return;
    };

    let current_state = *world.resource::<State<GameState>>().get();
    if pending_respawn && !matches!(current_state, GameState::DungeonSpawning) {
        info!("|HIGHLIGHT| Force state to DungeonSpawning due to respawn pending");
        let mut next_state: NextState<GameState> = NextState::default();
        next_state.set(GameState::DungeonSpawning);
        world.insert_resource(next_state);
    }

    info!("|HIGHLIGHT| Force transition from {current_state:?} to DungeonSpawning. Pending respawn {pending_respawn}");
    world.run_schedule(StateTransition);

    let current_state = world.resource::<State<GameState>>();
    info!("|HIGHLIGHT| State transitioned to {current_state:?}");
}
