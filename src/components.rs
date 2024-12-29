mod checksum_transform;
mod healing;
mod move_throttle;

pub use checksum_transform::checksum_transform;
pub use healing::Healing;
pub use move_throttle::{checksum_move_throttle, MoveThrottle};

use bevy::prelude::*;

#[derive(Component)]
pub struct ExitTile;

#[derive(Component)]
pub struct FloorTile;

#[derive(Component)]
pub struct Health {
    pub current: u8,
    pub max: u8,
}

impl Health {
    pub fn new( max: u8) -> Self {
        Self { current: max, max }
    }
}

#[derive(Component)]
pub struct HealthBar;

#[derive(Clone, Component, Copy, Debug)]
pub struct Monster;

/// An entity that can interfere with a player's intended movement
#[derive(Component, Clone, Copy)]
pub enum Obstacle {
    Monster,
    Player,
    Wall,
}

#[derive(Clone, Component, Copy, Debug)]
pub struct Player {
    pub id: PlayerId,
}

pub type PlayerId = usize;

#[derive(Component)]
pub struct WallTile;
