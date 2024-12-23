use crate::resources::config;
use bevy::prelude::*;

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
pub struct PlayerMovement {
    pub direction: Option<Vec2>,
    pub throttle: Timer,
}

impl Default for PlayerMovement {
    fn default() -> Self {
        Self {
            direction: None,
            throttle: Timer::from_seconds(config::PLAYER_MOVE_THROTTLE_SECONDS, TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub struct WallTile;
