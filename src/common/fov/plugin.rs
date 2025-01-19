use super::{events::FovEventsPlugin, recalculate_fov::recalculate_fov};
use crate::{dungeon, hud, prelude::*};
use bevy_ggrs::GgrsSchedule;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct FovCoreSet;

pub struct FovPlugin;

impl Plugin for FovPlugin {
    fn build(&self, app: &mut App) {
        let core_systems = recalculate_fov
            .in_set(FovCoreSet)
            .run_if(in_state(GameState::InGame))
            .ambiguous_with(hud::HudCoreSet)
            .ambiguous_with(dungeon::DungeonCoreSet);

        if game_mode(GameMode::SinglePlayer) {
            app.add_systems(
                Update,
                core_systems.run_if(|| game_mode(GameMode::SinglePlayer)),
            );
        } else {
            app.add_systems(GgrsSchedule, core_systems);
        }

        app.add_plugins(FovEventsPlugin);
    }
}
