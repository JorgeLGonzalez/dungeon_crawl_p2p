use crate::resources::config;
use bevy::math::{Vec2, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct DungeonPosition {
    pub x: isize,
    pub y: isize,
}

impl DungeonPosition {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: Self) -> f32 {
        self.to_vec2().distance(other.to_vec2())
    }

    pub fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }

    pub fn to_vec3(&self, z: f32) -> Vec3 {
        self.to_vec2().extend(z)
    }
}

impl From<DungeonPosition> for Vec2 {
    fn from(pos: DungeonPosition) -> Self {
        pos.to_vec2()
    }
}

impl From<DungeonPosition> for Vec3 {
    fn from(pos: DungeonPosition) -> Self {
        pos.to_vec3(config::MAP_Z_LAYER)
    }
}
