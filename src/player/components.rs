mod move_throttle;

pub use move_throttle::MoveThrottle;

use crate::items::MagicItem;
use bevy::prelude::Component;

#[derive(Component, Clone, Hash)]
pub struct Inventory {
    pub items: Vec<MagicItem>,
}

impl Inventory {
    pub fn new() -> Self {
        Self { items: vec![] }
    }
}

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
