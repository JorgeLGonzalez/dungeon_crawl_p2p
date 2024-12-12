use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub id: usize,
}

#[derive(Component)]
pub struct Tile;

#[derive(Clone, Component, Copy)]
pub struct MoveDir(pub Vec2);
