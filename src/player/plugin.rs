use super::{
    camera::*, components::*, events::*, player_actions::*, spawn_players, teleport_players,
};
use crate::{
    common,
    config::{game_mode, GameMode},
    dungeon::{DungeonCoreSet, SpawnDungeonSet},
    monsters::MonstersCoreSet,
    GameState,
};
use bevy::prelude::*;
use bevy_ggrs::{GgrsApp, ReadInputs};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PlayerCoreSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SpawnPlayersSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::DungeonSpawning),
            (teleport_players, spawn_players, setup_camera)
                .in_set(SpawnPlayersSet)
                .chain()
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
            exit_level.run_if(on_event::<PlayerMovesEvent>),
        )
            .in_set(PlayerCoreSet)
            .chain()
            .ambiguous_with(DungeonCoreSet)
            .before(MonstersCoreSet);

        common::add_core_systems(app, core_systems);

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
