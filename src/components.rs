use crate::resources::config;
use bevy::prelude::*;
use std::hash::{Hash, Hasher};
use std::time::Duration;

#[derive(Component)]
pub struct ExitTile;

#[derive(Component)]
pub struct FloorTile;

#[derive(Clone, Component, Copy, Debug)]
pub struct Monster;

#[derive(Clone, Component, Copy, Debug)]
pub struct Player {
    pub id: usize,
}

#[derive(Clone, Component, Debug)]
pub struct MoveThrottle(Timer);

impl MoveThrottle {
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

#[derive(Component)]
pub struct WallTile;
