mod healing;
mod health;
mod move_throttle;

pub use healing::Healing;
pub use health::{Health, HealthUnit};
pub use move_throttle::MoveThrottle;

use bevy::{prelude::*, utils::hashbrown::HashMap};

#[derive(Component)]
pub struct Damage(pub DamageUnit);

pub type DamageUnit = u8;

#[derive(Component)]
pub struct FieldOfView {
    pub radius: FovRadius,
    pub visible_tiles: FovTileMap,
}

impl FieldOfView {
    pub fn new(radius: FovRadius) -> Self {
        Self {
            radius,
            visible_tiles: FovTileMap::default(),
        }
    }
}

pub type FovRadius = u8;
pub type FovTileMap = HashMap<IVec2, Entity>;

/// An entity that can interfere with a player's intended movement
#[derive(Component, Clone, Copy)]
pub enum Obstacle {
    Monster,
    Player,
    Wall,
}
