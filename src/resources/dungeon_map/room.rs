use super::dungeon_position::DungeonPosition;

pub struct Room {
    pub x: isize,
    pub y: isize,
    pub width: usize,
    pub height: usize,
}

impl Room {
    pub fn new(x: isize, y: isize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn overlaps(&self, other: &Room) -> bool {
        self.left() < other.right()
            && self.right() >= other.left()
            && self.top() <= other.bottom()
            && self.bottom() > other.top()
    }

    pub fn bottom(&self) -> isize {
        self.y + self.height as isize
    }

    pub fn left(&self) -> isize {
        self.x
    }

    pub fn right(&self) -> isize {
        self.x + self.width as isize
    }

    pub fn tile_positions(&self) -> impl Iterator<Item = DungeonPosition> {
        let v_range = self.top()..self.bottom();

        (self.left()..self.right())
            .flat_map(move |x| v_range.clone().map(move |y| DungeonPosition::new(x, y)))
    }

    pub fn top(&self) -> isize {
        self.y
    }
}
