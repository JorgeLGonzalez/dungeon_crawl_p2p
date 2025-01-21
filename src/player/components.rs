mod move_throttle;

pub use move_throttle::MoveThrottle;

use bevy::prelude::Component;

/// An entity that can interfere with a player's intended movement
#[derive(Component, Clone, Copy, Hash)]
pub enum Obstacle {
    Monster,
    Player,
    Wall,
}

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Clone, Component, Copy, Debug, Hash)]
pub struct Player {
    pub id: PlayerId,
}

pub type PlayerId = usize;
