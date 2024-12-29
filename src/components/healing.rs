use crate::resources::config;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Healing(Timer);

impl Healing {
    pub fn just_finished(&self) -> bool {
        self.0.just_finished()
    }

    pub fn tick(&mut self, delta: Duration) {
        self.0.tick(delta);
    }
}

impl Default for Healing {
    fn default() -> Self {
        Self(Timer::from_seconds(
            config::PLAYER_HEALING_SECONDS,
            TimerMode::Repeating,
        ))
    }
}
