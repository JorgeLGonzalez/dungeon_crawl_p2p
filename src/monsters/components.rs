mod monster;

use bevy::prelude::Component;
pub use monster::{Monster, MonsterBundle, MonsterTemplate};

/// LastAction is used to track the time of the last action of a monster so as
/// to throttle the rate at which monsters can act.
#[derive(Component, Clone, Copy)]
pub struct LastAction {
    pub time: f32,
}

impl LastAction {
    pub fn new() -> Self {
        // Monsters will act immediately upon spawning since we need to wait for
        // the GgrsSchedule Time to synchronize among clients
        Self { time: 0. }
    }
}
