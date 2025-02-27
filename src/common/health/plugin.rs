use super::{drink_potion::drink_potion, events::HealthEventsPlugin, healing::healing, *};
use crate::{common, monsters::MonstersCoreSet, player::PlayerCoreSet, prelude::*};
use bevy_ggrs::GgrsApp;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        common::add_core_systems(
            app,
            (healing, drink_potion)
                .chain()
                .after(PlayerCoreSet)
                .before(MonstersCoreSet),
        );

        if !game_mode(GameMode::SinglePlayer) {
            app.rollback_component_with_copy::<Damage>()
                .checksum_component_with_hash::<Damage>()
                .rollback_component_with_clone::<Healing>()
                .rollback_component_with_copy::<Health>()
                .checksum_component_with_hash::<Health>();
        }

        app.add_plugins(HealthEventsPlugin);
    }
}
