use super::DungeonPosition;

#[derive(Debug, Eq, PartialEq)]
pub struct AStarNode {
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
