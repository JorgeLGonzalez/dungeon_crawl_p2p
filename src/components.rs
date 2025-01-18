mod move_throttle;

pub use move_throttle::MoveThrottle;

use bevy::prelude::*;

/// An entity that can interfere with a player's intended movement
#[derive(Component, Clone, Copy)]
pub enum Obstacle {
    Monster,
    Player,
    Wall,
}
