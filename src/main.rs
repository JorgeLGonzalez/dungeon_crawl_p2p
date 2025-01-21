mod common;
pub mod config;
mod dungeon;
mod game_states;
mod hud;
mod items;
mod monsters;
mod player;
mod startup;

pub use common::{fov, health};

pub mod prelude {
    pub use crate::common::RandomGenerator;
    pub use crate::config::{self, game_mode, GameMode};
    pub use crate::dungeon::DungeonMap;
    pub use crate::fov::FieldOfView;
    pub use crate::game_states::GameState;
    pub use crate::health::{Damage, DamageUnit, Healing, Health, HealthUnit};
    pub use crate::player::Player;
    pub use bevy::prelude::*;
}

use bevy::log::LogPlugin;
use bevy_ggrs::GgrsPlugin;
use prelude::*;

fn main() {
    let logging_filter = if config::GGRS_DEBUG {
        "dungeon_crawl_p2p=trace,bevy_ggrs=trace,ggrs=trace,ggrs::network=info".to_string()
    } else {
        "dungeon_crawl_p2p=debug".to_string()
    };

    App::new()
        .add_plugins((
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
                    filter: logging_filter,
                    ..default()
                }),
            common::CommonPlugin,
            dungeon::DungeonPlugin,
            fov::FovPlugin,
            health::HealthPlugin,
            hud::HudPlugin,
            items::ItemsPlugin,
            game_states::GameStatesPlugin,
            GgrsPlugin::<config::GgrsSessionConfig>::default(),
            monsters::MonstersPlugin,
            player::PlayerPlugin,
            startup::StartupPlugin,
        ))
        .run();
}
