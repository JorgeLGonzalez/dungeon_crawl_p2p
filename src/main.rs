mod components;
mod dungeon;
mod events;
mod hud;
mod monsters;
mod player;
mod resources;
mod startup;
mod systems;

use bevy::{log::LogPlugin, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_ggrs::{GgrsApp, GgrsPlugin, GgrsSchedule};
use components::{checksum_transform, Healing, Health, MoveThrottle};
use resources::{
    assets::FontAssets,
    config::{self, GameMode, GAME_MODE},
    RandomGenerator,
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
        dungeon::DungeonPlugin,
        hud::HudPlugin,
        GgrsPlugin::<config::GgrsSessionConfig>::default(),
        monsters::MonstersPlugin,
        player::PlayerPlugin,
        startup::StartupPlugin,
    ))
    .init_state::<GameState>()
    .add_loading_state(
        LoadingState::new(GameState::Loading)
            .continue_to_state(GameState::Startup)
            .load_collection::<FontAssets>(),
    );

    add_events(&mut app);

    app.add_systems(OnEnter(GameState::Startup), startup)
        .add_systems(OnEnter(GameState::GameOver), game_over);

    // systems used in both Single Player Update schedule and GgrsScheduled
    let core_systems = (healing, recalculate_fov)
        .chain()
        .after(player::PlayerCoreSet)
        .after(monsters::MonstersCoreSet)
        .before(dungeon::DungeonCoreSet)
        .before(hud::HudCoreSet)
        .run_if(in_state(GameState::InGame));

    if game_mode(GameMode::SinglePlayer) {
        app.add_systems(
            Update,
            core_systems.run_if(|| game_mode(GameMode::SinglePlayer)),
        );
    } else {
        ggrs_setup(&mut app);

        app.add_systems(GgrsSchedule, core_systems).add_systems(
            Update,
            (
                create_p2p_session
                    .run_if(in_state(GameState::Startup).and(|| game_mode(GameMode::MultiPlayer))),
                handle_ggrs_events.run_if(in_state(GameState::InGame)),
            ),
        );
    }

    app.run();
}

fn add_events(app: &mut App) {
    app.add_event::<events::RecalculateFovEvent>()
        .add_event::<events::SnapshotStateEvent>();
}

/// Register components and resources for GGRS snapshots and rollback
fn ggrs_setup(app: &mut App) {
    app.rollback_component_with_clone::<Healing>()
        .rollback_component_with_clone::<MoveThrottle>()
        .rollback_component_with_clone::<Transform>()
        .rollback_component_with_copy::<Health>()
        .rollback_component_with_copy::<monsters::LastAction>()
        .rollback_component_with_copy::<monsters::Monster>()
        .rollback_component_with_copy::<player::Player>()
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
