mod move_throttle;

pub use move_throttle::{checksum_move_throttle, MoveThrottle};

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

#[derive(Component)]
pub struct WallTile;
