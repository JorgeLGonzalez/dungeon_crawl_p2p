mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use bevy_ggrs::{checksum_hasher, GgrsApp, GgrsPlugin, GgrsSchedule, ReadInputs};
use components::Player;
use resources::config;
use std::hash::{Hash, Hasher};
use systems::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    title: "Dungeon Crawl".to_string(),
                    ..default()
                }),
                ..default()
            }),
            GgrsPlugin::<config::GgrsSessionConfig>::default(),
        ))
        .init_state::<GameState>()
        // .rollback_component_with_clone::<GlobalTransform>()
        // .rollback_component_with_clone::<InheritedVisibility>()
        .rollback_component_with_clone::<Transform>()
        // .rollback_component_with_clone::<ViewVisibility>()
        // .rollback_component_with_clone::<Visibility>()
        // .rollback_component_with_copy::<MoveDir>()
        .rollback_component_with_copy::<Player>()
        .checksum_component::<Transform>(checksum_transform)
        .add_systems(
            OnEnter(GameState::MatchMaking),
            (
                spawn_camera,
                start_matchbox_socket.run_if(|| config::P2P_MODE),
            ),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            (spawn_dungeon, spawn_players).chain(),
        )
        .add_systems(
            Update,
            (
                (
                    create_p2p_session.run_if(|| config::P2P_MODE),
                    start_sync_test_session.run_if(|| !config::P2P_MODE),
                )
                    .run_if(in_state(GameState::MatchMaking)),
                handle_ggrs_events.run_if(in_state(GameState::InGame)),
            ),
        )
        .add_systems(ReadInputs, read_local_inputs)
        .add_systems(
            GgrsSchedule,
            (move_players, camera_follow)
                .chain()
                .run_if(in_state(GameState::InGame)),
        )
        .run();
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
    #[default]
    MatchMaking,
    InGame,
}
