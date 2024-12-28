mod components;
mod events;
mod resources;
mod systems;

use bevy::{log::LogPlugin, prelude::*};
use bevy_ggrs::{GgrsApp, GgrsPlugin, GgrsSchedule, ReadInputs};
use components::{checksum_move_throttle, checksum_transform, Monster, MoveThrottle, Player};
use resources::{
    checksum_rng,
    config::{self, GameMode, GAME_MODE},
    DesyncEvent, MonsterMoveTracker, RandomGenerator,
};
use std::hash::Hash;
use systems::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    title: "Dungeon Crawl".to_string(),
                    ..default()
                }),
                ..default()
            })
            .set(LogPlugin {
                filter: "dungeon_crawl_p2p=debug".to_string(),
                // filter: "bevy_ggrs=trace,ggrs=trace,ggrs::network=info".to_string(),
                ..default()
            }),
        GgrsPlugin::<config::GgrsSessionConfig>::default(),
    ))
    .init_state::<GameState>();

    app.init_resource::<MonsterMoveTracker>()
        .add_event::<DesyncEvent>()
        .add_event::<events::MonsterAttacksEvent>()
        .add_event::<events::MonsterMovesEvent>()
        .add_event::<events::PlayerAttacksEvent>()
        .add_event::<events::PlayerMovesEvent>()
        .add_event::<events::PlayerMoveIntentEvent>()
        .add_event::<events::SnapshotStateEvent>()
        .add_event::<events::StopMovingEvent>();

    // Register components and resources for GGRS snapshots and rollback
    app
        // .rollback_component_with_clone::<GlobalTransform>()
        // .rollback_component_with_clone::<InheritedVisibility>()
        .rollback_component_with_clone::<MoveThrottle>()
        .rollback_component_with_clone::<Transform>()
        // .rollback_component_with_clone::<ViewVisibility>()
        // .rollback_component_with_clone::<Visibility>()
        .rollback_component_with_copy::<Monster>()
        .rollback_component_with_copy::<Player>()
        .rollback_resource_with_clone::<RandomGenerator>()
        .checksum_component::<MoveThrottle>(checksum_move_throttle)
        .checksum_component::<Transform>(checksum_transform)
        .checksum_resource::<RandomGenerator>(checksum_rng);

    app.add_systems(OnEnter(GameState::Startup), (spawn_camera, startup))
        .add_systems(
            OnEnter(GameState::InGame),
            (spawn_dungeon, spawn_players, spawn_monsters).chain(),
        )
        .add_systems(
            Update,
            (
                create_p2p_session.run_if(
                    in_state(GameState::Startup).and(|| GAME_MODE == GameMode::MultiPlayer),
                ),
                (
                    handle_ggrs_events.run_if(
                        in_state(GameState::InGame).and(|| GAME_MODE != GameMode::SinglePlayer),
                    ),
                    (
                        do_player_action,
                        tick_move_throttle,
                        stop_moving,
                        handle_move_intent,
                        attack_monster,
                        move_player,
                        move_camera,
                        do_monsters_action,
                        attack_player,
                        move_monster,
                    )
                        .chain()
                        .run_if(|| GAME_MODE == GameMode::SinglePlayer),
                )
                    .run_if(in_state(GameState::InGame)),
            ),
        )
        .add_systems(ReadInputs, read_player_inputs)
        .add_systems(
            GgrsSchedule,
            (
                (
                    // below should follow same order as single player mode Update
                    do_player_action,
                    tick_move_throttle,
                    stop_moving,
                    handle_move_intent,
                    attack_monster,
                    move_player,
                    move_camera,
                    do_monsters_action,
                    attack_player,
                    move_monster,
                )
                    .chain()
                    .run_if(in_state(GameState::InGame)),
                persist_monster_moves,
            )
                .chain(),
        )
        .add_systems(OnEnter(GameState::GameOver), game_over);

    app.run();
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
enum GameState {
    GameOver,
    InGame,
    Paused,
    #[default]
    Startup,
}
