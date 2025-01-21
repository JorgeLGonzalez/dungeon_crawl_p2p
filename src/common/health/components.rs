use crate::config;
use bevy::prelude::*;
use std::hash::Hash;
use std::time::Duration;

#[derive(Component, Clone, Copy, Hash)]
pub struct Damage(pub DamageUnit);

pub type DamageUnit = u8;

#[derive(Component, Clone)]
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

#[derive(Component, Clone, Copy, Hash)]
pub struct Health {
    pub current: HealthUnit,
    pub max: HealthUnit,
}

impl Health {
    pub fn new(max: HealthUnit) -> Self {
        Self { current: max, max }
    }
}

pub type HealthUnit = u8;
