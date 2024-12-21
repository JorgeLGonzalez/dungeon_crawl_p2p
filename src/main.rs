mod components;
mod resources;
mod systems;

use bevy::{log::LogPlugin, prelude::*};
use bevy_ggrs::{checksum_hasher, GgrsApp, GgrsPlugin, GgrsSchedule, ReadInputs};
use components::{Monster, Player, PlayerMovement};
use resources::{
    config::{self, GameMode, GAME_MODE},
    RandomGenerator,
};
use std::hash::{Hash, Hasher};
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
                // level: bevy::log::Level::TRACE,
                ..default()
            }),
        GgrsPlugin::<config::GgrsSessionConfig>::default(),
    ))
    .init_state::<GameState>();

    // Register components and resources for GGRS snapshots and rollback
    app
        // .rollback_component_with_clone::<GlobalTransform>()
        // .rollback_component_with_clone::<InheritedVisibility>()
        .rollback_component_with_clone::<PlayerMovement>()
        .rollback_component_with_clone::<Transform>()
        // .rollback_component_with_clone::<ViewVisibility>()
        // .rollback_component_with_clone::<Visibility>()
        .rollback_component_with_copy::<Monster>()
        .rollback_component_with_copy::<Player>()
        .rollback_resource_with_clone::<RandomGenerator>()
        .checksum_component::<Transform>(checksum_transform);

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
                    handle_ggrs_events.run_if(|| GAME_MODE != GameMode::SinglePlayer),
                    (move_single_player, move_camera, move_monsters)
                        .chain()
                        .run_if(|| GAME_MODE == GameMode::SinglePlayer),
                )
                    .run_if(in_state(GameState::InGame)),
            ),
        )
        .add_systems(ReadInputs, read_local_inputs)
        .add_systems(
            GgrsSchedule,
            (move_players, move_camera, move_monsters)
                .chain()
                .run_if(in_state(GameState::InGame)),
        );

    app.run();
}

// See https://johanhelsing.studio/posts/extreme-bevy-desync-detection
fn checksum_transform(transform: &Transform) -> u64 {
    let mut hasher = checksum_hasher();
    assert!(
        transform.is_finite(),
        "Hashing is not stable for NaN f32 value."
    );

    transform.translation.x.to_bits().hash(&mut hasher);
    transform.translation.y.to_bits().hash(&mut hasher);
    transform.translation.z.to_bits().hash(&mut hasher);

    transform.rotation.x.to_bits().hash(&mut hasher);
    transform.rotation.y.to_bits().hash(&mut hasher);
    transform.rotation.z.to_bits().hash(&mut hasher);
    transform.rotation.w.to_bits().hash(&mut hasher);

    hasher.finish()
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
enum GameState {
    InGame,
    #[default]
    Startup,
}
