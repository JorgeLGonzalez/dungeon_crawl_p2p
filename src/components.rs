mod checksum_transform;
mod move_throttle;

pub use checksum_transform::checksum_transform;
pub use move_throttle::{checksum_move_throttle, MoveThrottle};

use bevy::prelude::*;

#[derive(Component)]
pub struct ExitTile;

#[derive(Component)]
pub struct FloorTile;

#[derive(Clone, Component, Copy, Debug)]
pub struct Monster;

#[derive(Component)]
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
