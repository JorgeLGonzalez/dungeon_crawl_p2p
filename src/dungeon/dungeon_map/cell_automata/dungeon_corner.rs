use super::{DungeonPosition, X_MAX, X_MIN, Y_MAX, Y_MIN};
use crate::common::RandomGenerator;

/// Each corner of the dungeon within the perimeter (i.e. excluding the border
/// walls)
#[derive(Clone, Copy)]
pub(super) enum DungeonCorner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl DungeonCorner {
    pub fn random(rng: &mut RandomGenerator) -> Self {
        match rng.gen_range(0..4) {
            0 => DungeonCorner::BottomLeft,
            1 => DungeonCorner::BottomRight,
            2 => DungeonCorner::TopLeft,
            3 => DungeonCorner::TopRight,
            _ => unreachable!(),
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            DungeonCorner::BottomLeft => DungeonCorner::TopRight,
            DungeonCorner::BottomRight => DungeonCorner::TopLeft,
            DungeonCorner::TopLeft => DungeonCorner::BottomRight,
            DungeonCorner::TopRight => DungeonCorner::BottomLeft,
        }
    }

    pub fn pos(&self) -> DungeonPosition {
        match self {
            DungeonCorner::BottomLeft => DungeonPosition::new(X_MIN + 1, Y_MIN + 1),
            DungeonCorner::BottomRight => DungeonPosition::new(X_MAX - 1, Y_MIN + 1),
            DungeonCorner::TopLeft => DungeonPosition::new(X_MIN + 1, Y_MAX - 1),
            DungeonCorner::TopRight => DungeonPosition::new(X_MAX - 1, Y_MAX - 1),
        }
    }
}
