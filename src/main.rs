mod components;
mod events;
mod hud;
mod player;
mod resources;
mod systems;

use bevy::{log::LogPlugin, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_ggrs::{GgrsApp, GgrsPlugin, GgrsSchedule, ReadInputs};
use components::{checksum_transform, Healing, Health, LastAction, Monster, MoveThrottle, Player};
use resources::{
    assets::FontAssets,
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
    .init_state::<GameState>()
    .add_loading_state(
        LoadingState::new(GameState::Loading)
            .continue_to_state(GameState::Startup)
            .load_collection::<FontAssets>(),
    )
    .init_resource::<MonsterMoveTracker>();

    add_events(&mut app);

    app.add_systems(
        OnEnter(GameState::Startup),
        (hud::setup_camera, player::setup_camera, startup),
    )
    .add_systems(
        OnEnter(GameState::InGame),
        (
            spawn_dungeon,
            spawn_players,
            hud::setup_health_bar,
            spawn_monsters,
        )
            .chain(),
    )
    .add_systems(OnEnter(GameState::GameOver), game_over);

    // systems used in both Single Player Update schedule and GgrsScheduled
    let core_systems = (
        do_player_action,
        tick_move_throttle,
        healing,
        stop_moving,
        handle_move_intent,
        attack_monster,
        move_player,
        player::follow_with_camera,
        do_monsters_action,
        attack_player,
        move_monster,
        update_last_action,
        recalculate_fov,
        hud::health_bar,
    )
        .chain()
        .run_if(in_state(GameState::InGame));

    if game_mode(GameMode::SinglePlayer) {
        app.add_systems(
            Update,
            core_systems.run_if(|| game_mode(GameMode::SinglePlayer)),
        );
    } else {
        ggrs_setup(&mut app);

        app.add_systems(ReadInputs, read_player_inputs)
            .add_systems(GgrsSchedule, core_systems)
            .add_systems(GgrsSchedule, persist_monster_moves.after(move_monster))
            .add_systems(
                Update,
                (
                    create_p2p_session.run_if(
                        in_state(GameState::Startup).and(|| game_mode(GameMode::MultiPlayer)),
                    ),
                    handle_ggrs_events.run_if(in_state(GameState::InGame)),
                ),
            );
    }

    app.run();
}

fn add_events(app: &mut App) {
    app.add_event::<DesyncEvent>()
        .add_event::<events::MonsterActedEvent>()
        .add_event::<events::MonsterAttacksEvent>()
        .add_event::<events::MonsterMovesEvent>()
        .add_event::<events::PlayerAttacksEvent>()
        .add_event::<events::PlayerMovesEvent>()
        .add_event::<events::PlayerMoveIntentEvent>()
        .add_event::<events::RecalculateFovEvent>()
        .add_event::<events::SnapshotStateEvent>()
        .add_event::<events::StopMovingEvent>();
}

/// Register components and resources for GGRS snapshots and rollback
fn ggrs_setup(app: &mut App) {
    app
        // .rollback_component_with_clone::<GlobalTransform>()
        // .rollback_component_with_clone::<InheritedVisibility>()
        .rollback_component_with_clone::<Healing>()
        .rollback_component_with_clone::<MoveThrottle>()
        .rollback_component_with_clone::<Transform>()
        // .rollback_component_with_clone::<ViewVisibility>()
        // .rollback_component_with_clone::<Visibility>()
        .rollback_component_with_copy::<Health>()
        .rollback_component_with_copy::<LastAction>()
        .rollback_component_with_copy::<Monster>()
        .rollback_component_with_copy::<Player>()
        .rollback_resource_with_clone::<RandomGenerator>()
        .checksum_component::<Transform>(checksum_transform)
        .checksum_component_with_hash::<Health>()
        .checksum_component_with_hash::<MoveThrottle>()
        .checksum_resource_with_hash::<RandomGenerator>();
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
enum GameState {
    GameOver,
    InGame,
    #[default]
    Loading,
    Paused,
    Startup,
}

fn game_mode(mode: GameMode) -> bool {
    GAME_MODE == mode
}
