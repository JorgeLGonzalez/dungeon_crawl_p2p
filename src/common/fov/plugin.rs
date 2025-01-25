use super::{events::FovEventsPlugin, recalculate_fov::recalculate_fov};
use crate::{common, dungeon, hud, prelude::*};
use bevy_ggrs::GgrsApp;

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

        common::add_core_systems(app, core_systems);

        if !game_mode(GameMode::SinglePlayer) {
            app.rollback_component_with_clone::<FieldOfView>();
        }

        app.add_plugins(FovEventsPlugin);
    }
}
