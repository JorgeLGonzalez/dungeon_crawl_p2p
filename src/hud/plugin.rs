use super::*;
use crate::{common, dungeon::SpawnDungeonSet, player::PlayerMovesEvent, prelude::*};

#[derive(SystemSet, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct HudStartupSet;

#[derive(SystemSet, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct HudCoreSet;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TooltipPlugin)
            .add_systems(
                OnEnter(GameState::Startup),
                (
                    setup_camera,
                    setup_health_bar,
                    spawn_inventory_ui,
                    spawn_weapon_ui,
                    spawn_level_ui,
                    spawn_location_ui,
                )
                    .chain()
                    .in_set(HudStartupSet),
            )
            .add_systems(
                OnEnter(GameState::DungeonSpawning),
                (update_level_ui, update_location_ui)
                    .chain()
                    .after(SpawnDungeonSet),
            );

        common::add_core_systems(
            app,
            (
                health_bar,
                update_inventory,
                wield_weapon,
                update_location_ui.run_if(on_event::<PlayerMovesEvent>),
            )
                .chain()
                .after(TooltipCoreSet)
                .in_set(HudCoreSet),
        );
    }
}
