mod checksum_transform;
mod healing;
mod health;
mod move_throttle;

pub use checksum_transform::checksum_transform;
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

#[derive(Component, Clone, Copy)]
pub struct LastAction {
    pub time: f32,
}

impl LastAction {
    pub fn new() -> Self {
        // Monsters will act immediately upon spawning since we need to wait for
        // the GgrsSchedule Time to synchronize among clients
        Self { time: 0. }
    }
}

#[derive(Clone, Component, Copy, Debug)]
pub struct Monster;

#[derive(Component)]
pub enum MonsterType {
    Ettin,
    Goblin,
    Ogre,
    Orc,
}

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
