use super::{DungeonPosition, X_MAX, X_MIN, Y_MAX, Y_MIN};

/// Each corner of the dungeon within the perimeter (i.e. excluding the border
/// walls)
pub(super) enum DungeonCorner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl DungeonCorner {
    pub fn pos(&self) -> DungeonPosition {
        match self {
            DungeonCorner::BottomLeft => DungeonPosition::new(X_MIN + 1, Y_MIN + 1),
            DungeonCorner::BottomRight => DungeonPosition::new(X_MAX - 1, Y_MIN + 1),
            DungeonCorner::TopLeft => DungeonPosition::new(X_MIN + 1, Y_MAX - 1),
            DungeonCorner::TopRight => DungeonPosition::new(X_MAX - 1, Y_MAX - 1),
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
}
