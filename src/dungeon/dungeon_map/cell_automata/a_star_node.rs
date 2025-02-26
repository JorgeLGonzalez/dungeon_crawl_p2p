use super::DungeonPosition;

/// A node in the A* search algorithm, representing a position in the dungeon
/// and its estimated cost.
#[derive(Debug, Eq, PartialEq)]
pub struct AStarNode {
    /// The cost of reaching this node from the start plus the manhattan distance
    /// to the goal. This estimated cost is used to sort the open set of nodes
    /// pending evaluation.
    pub cost: usize,
    pub pos: DungeonPosition,
}

impl AStarNode {
    pub fn new(pos: DungeonPosition, cost: usize) -> Self {
        Self { cost, pos }
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
