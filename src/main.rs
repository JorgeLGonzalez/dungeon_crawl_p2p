mod common;
mod dungeon;
mod game_states;
mod hud;
mod monsters;
mod player;
mod startup;

pub use common::{fov, health};
pub use startup::{assets, config};

use bevy::{log::LogPlugin, prelude::*};
use bevy_ggrs::{GgrsApp, GgrsPlugin};
use game_states::GameState;
use startup::config::{game_mode, GameMode};

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
        fov::FovPlugin,
        health::HealthPlugin,
        hud::HudPlugin,
        game_states::GameStatesPlugin,
        GgrsPlugin::<config::GgrsSessionConfig>::default(),
        monsters::MonstersPlugin,
        player::PlayerPlugin,
        startup::StartupPlugin,
    ));

    if !game_mode(GameMode::SinglePlayer) {
        ggrs_setup(&mut app);
    }

    app.run();
}

/// Register components and resources for GGRS snapshots and rollback
fn ggrs_setup(app: &mut App) {
    // app.rollback_component_with_clone::<Healing>()
    app.rollback_component_with_clone::<Transform>()
        .rollback_component_with_copy::<monsters::LastAction>()
        .rollback_component_with_copy::<monsters::Monster>()
        .rollback_component_with_copy::<player::Player>();
}
