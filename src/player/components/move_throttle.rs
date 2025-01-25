use crate::config;
use bevy::{
    prelude::Component,
    time::{Timer, TimerMode},
};
use std::{
    hash::{Hash, Hasher},
    time::Duration,
};

#[derive(Clone, Component, Debug)]
pub struct MoveThrottle(Timer);

impl MoveThrottle {
    pub fn elapsed_secs(&self) -> f32 {
        self.0.elapsed_secs()
    }

    pub fn just_finished(&self) -> bool {
        self.0.just_finished()
    }

    pub fn tick(&mut self, delta: Duration) {
        self.0.tick(delta);
    }
}

impl Default for MoveThrottle {
    fn default() -> Self {
        Self(Timer::from_seconds(
            config::PLAYER_MOVE_THROTTLE_SECONDS,
            TimerMode::Once,
        ))
    }
}

impl Hash for MoveThrottle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{:?}", self.0).hash(state);
    }
}
