mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use bevy_ggrs::{GgrsPlugin, ReadInputs};
use resources::config;
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
        // .rollback_component_with_clone::<Transform>()
        .add_systems(
            OnEnter(GameState::MatchMaking),
            (spawn_camera, start_matchbox_socket),
        )
        .add_systems(
            Update,
            wait_for_players.run_if(in_state(GameState::MatchMaking)),
        )
        .add_systems(OnEnter(GameState::InGame), spawn_dungeon)
        .add_systems(ReadInputs, read_local_inputs)
        .run();
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
enum GameState {
    #[default]
    MatchMaking,
    InGame,
}
