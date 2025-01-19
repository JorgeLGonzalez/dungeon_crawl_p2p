use crate::prelude::*;
use bevy_ggrs::GgrsSchedule;

/// Add core [`GameState::InGame`] systems to the Update or GgrsSchedule schedule
/// (depending on [`GameMode`]). This is a helper function used by various
/// module plugins.
pub fn add_core_systems(app: &mut App, systems: impl IntoSystemConfigs<()>) {
    let core_systems = systems.run_if(in_state(GameState::InGame));

    if game_mode(GameMode::SinglePlayer) {
        app.add_systems(Update, core_systems);
    } else {
        app.add_systems(GgrsSchedule, core_systems);
    }
}
