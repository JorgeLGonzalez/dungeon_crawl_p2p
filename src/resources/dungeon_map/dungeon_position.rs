use bevy::math::{Vec2, Vec3};

use crate::resources::config;

#[derive(Clone, Copy, Debug)]
pub struct DungeonPosition {
    pub x: isize,
    pub y: isize,
}

impl DungeonPosition {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl From<DungeonPosition> for Vec2 {
    fn from(pos: DungeonPosition) -> Self {
        Vec2::new(pos.x as f32, pos.y as f32)
    }
}

impl From<DungeonPosition> for Vec3 {
    fn from(pos: DungeonPosition) -> Self {
        Vec2::new(pos.x as f32, pos.y as f32).extend(config::MAP_Z_LAYER)
    }
}
