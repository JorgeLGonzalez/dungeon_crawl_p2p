use super::components::{Healing, Health};
use bevy::prelude::*;
use bevy_ggrs::GgrsApp;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.rollback_component_with_clone::<Healing>()
            .rollback_component_with_copy::<Health>()
            .checksum_component_with_hash::<Health>();
    }
}
