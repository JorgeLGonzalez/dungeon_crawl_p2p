use super::*;
use crate::{common, prelude::*};

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

        common::add_core_systems(
            app,
            (
                health_bar,
                update_inventory,
                tooltip.ambiguous_with(health_bar),
            )
                .in_set(HudCoreSet),
        );
    }
}
