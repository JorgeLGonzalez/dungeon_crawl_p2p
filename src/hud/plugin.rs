use super::*;
use crate::{common, prelude::*};

#[derive(SystemSet, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct HudStartupSet;

#[derive(SystemSet, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct HudCoreSet;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TooltipPlugin).add_systems(
            OnEnter(GameState::Startup),
            (
                setup_camera,
                setup_health_bar,
                spawn_inventory_ui,
                spawn_weapon_ui,
                spawn_level_ui,
            )
                .chain()
                .in_set(HudStartupSet),
        );

        common::add_core_systems(
            app,
            (health_bar, update_inventory, wield_weapon)
                .chain()
                .after(TooltipCoreSet)
                .in_set(HudCoreSet),
        );
    }
}
