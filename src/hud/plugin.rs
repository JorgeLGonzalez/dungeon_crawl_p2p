use super::{health_bar, setup_camera, setup_health_bar, spawn_tooltip, tooltip};
use crate::{game_mode, GameMode, GameState};
use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

#[derive(SystemSet, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct HudStartupSet;

#[derive(SystemSet, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct HudCoreSet;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Startup),
            (setup_camera, setup_health_bar, spawn_tooltip).in_set(HudStartupSet),
        );

        let core_systems = (health_bar, tooltip)
            .chain()
            .in_set(HudCoreSet)
            .run_if(in_state(GameState::InGame));

        if game_mode(GameMode::SinglePlayer) {
            app.add_systems(
                Update,
                core_systems.run_if(|| game_mode(GameMode::SinglePlayer)),
            );
        } else {
            app.add_systems(GgrsSchedule, core_systems);
        }
    }
}
