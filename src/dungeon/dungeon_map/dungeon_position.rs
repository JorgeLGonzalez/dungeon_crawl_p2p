use super::{MAP_Z_LAYER, X_MAX, X_MIN, Y_MAX, Y_MIN};
use bevy::math::{Vec2, Vec3};

/// A position in the dungeon, represented as a pair of x and y coordinates.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DungeonPosition {
    pub x: isize,
    pub y: isize,
}

impl DungeonPosition {
    pub fn from_vec2(pos: Vec2) -> Self {
        Self {
            x: pos.x as isize,
            y: pos.y as isize,
        }
    }

    pub fn from_vec3(pos: Vec3) -> Self {
        Self {
            x: pos.x as isize,
            y: pos.y as isize,
        }
    }

    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    /// Returns true if the position is at the perimeter of the dungeon.
    pub fn at_perimeter(&self) -> bool {
        self.x == X_MAX || self.x == X_MIN || self.y == Y_MAX || self.y == Y_MIN
    }

    pub fn distance(&self, other: Self) -> f32 {
        self.to_vec2().distance(other.to_vec2())
    }

    pub fn manhattan_distance(&self, other: Self) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }

    /// Returns an iterator over the perimeter of the square with the given radius
    /// from the current position. The perimeter is defined as the outermost tiles
    /// of the square, including corners.
    pub fn perimeter(&self, radius: isize) -> impl Iterator<Item = DungeonPosition> + use<'_> {
        let mut perimeter = vec![];

        for ix in -radius..=radius {
            // top row
            perimeter.push(DungeonPosition::new(self.x + ix, self.y + radius));
            // bottom row
            perimeter.push(DungeonPosition::new(self.x + ix, self.y - radius));
        }

        for iy in (-radius + 1)..radius {
            // right column
            perimeter.push(DungeonPosition::new(self.x + radius, self.y + iy));
            // left column
            perimeter.push(DungeonPosition::new(self.x - radius, self.y + iy));
        }

        perimeter.into_iter()
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
        pos.to_vec3(MAP_Z_LAYER)
    }
}

impl std::fmt::Display for DungeonPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}
