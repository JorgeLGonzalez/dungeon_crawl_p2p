use bevy::prelude::Component;
use std::hash::Hash;

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
