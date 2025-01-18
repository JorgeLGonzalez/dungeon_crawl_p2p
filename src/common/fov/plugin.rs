use super::{events::FovEventsPlugin, recalculate_fov::recalculate_fov};
use crate::{dungeon, hud, monsters, player, prelude::*};
use bevy_ggrs::GgrsSchedule;

pub struct FovPlugin;

impl Plugin for FovPlugin {
    fn build(&self, app: &mut App) {
        let core_systems = recalculate_fov
            .before(hud::HudCoreSet)
            .before(dungeon::DungeonCoreSet)
            .after(player::PlayerCoreSet)
            .after(monsters::MonstersCoreSet)
            .run_if(in_state(GameState::InGame));

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
