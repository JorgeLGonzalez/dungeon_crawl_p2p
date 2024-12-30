mod checksum_transform;
mod healing;
mod health;
mod move_throttle;

pub use checksum_transform::checksum_transform;
pub use healing::Healing;
pub use health::{Health, HealthUnit};
pub use move_throttle::MoveThrottle;

use bevy::prelude::*;

#[derive(Component)]
pub struct ExitTile;

#[derive(Component)]
pub struct FieldOfView {
    pub radius: FovRadius,
    pub visible_tiles: Vec<Vec2>,
}

impl FieldOfView {
    pub fn new(radius: FovRadius) -> Self {
        Self {
            radius,
            visible_tiles: vec![],
        }
    }
}

pub type FovRadius = u8;

#[derive(Component)]
pub struct FloorTile;

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
