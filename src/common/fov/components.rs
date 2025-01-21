use bevy::{prelude::*, utils::hashbrown::HashMap};

#[derive(Component, Clone)]
pub struct FieldOfView {
    pub radius: FovRadius,
    pub visible_tiles: FovTileMap,
}

impl FieldOfView {
    pub fn new(radius: FovRadius) -> Self {
        Self {
            radius,
            visible_tiles: FovTileMap::default(),
        }
    }
}

pub type FovRadius = u8;
pub type FovTileMap = HashMap<IVec2, Entity>;
