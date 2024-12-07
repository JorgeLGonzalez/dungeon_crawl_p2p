use super::config::{MAP_HEIGHT, MAP_WIDTH};
use bevy::math::Vec2;

#[derive(Clone, Copy)]
pub enum TileType {
    Exit,
    Floor,
    Wall,
}

const NUM_TILES: usize = MAP_WIDTH * MAP_HEIGHT;

pub struct DungeonMap {
    pub tiles: Vec<TileType>,
}

impl DungeonMap {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn idx_to_position(&self, index: usize) -> Vec2 {
        let idx = index as isize;
        const W: isize = MAP_WIDTH as isize;
        const H: isize = MAP_HEIGHT as isize;
        let x = (idx % W) - (W / 2);
        let y = (idx / W) - (H / 2);

        Vec2::new(x as f32, y as f32)
    }
}
