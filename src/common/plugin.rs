use super::{debug_ggrs::debug_ggrs, events::CommonEventsPlugin, fov::FovCoreSet};
use crate::prelude::*;
use bevy_ggrs::GgrsSchedule;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CommonEventsPlugin);

        if config::GGRS_DEBUG && !game_mode(GameMode::SinglePlayer) {
            app.add_systems(GgrsSchedule, debug_ggrs.after(FovCoreSet));
        }
    }
}
