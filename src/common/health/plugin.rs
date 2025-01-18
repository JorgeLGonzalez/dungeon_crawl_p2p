use super::{
    components::{Healing, Health},
    healing::healing,
};
use crate::{
    config::{game_mode, GameMode},
    monsters::MonstersCoreSet,
    player::PlayerCoreSet,
    GameState,
};
use bevy::prelude::*;
use bevy_ggrs::{GgrsApp, GgrsSchedule};

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        let core_systems = healing
            .run_if(in_state(GameState::InGame))
            .before(PlayerCoreSet)
            .before(MonstersCoreSet);

        if game_mode(GameMode::SinglePlayer) {
            app.add_systems(Update, core_systems);
        } else {
            app.rollback_component_with_clone::<Healing>()
                .rollback_component_with_copy::<Health>()
                .checksum_component_with_hash::<Health>();

            app.add_systems(GgrsSchedule, core_systems);
        }
    }
}
