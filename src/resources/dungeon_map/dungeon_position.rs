#[derive(Debug)]
pub struct DungeonPosition {
    pub x: isize,
    pub y: isize,
}

impl DungeonPosition {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}
