use bevy::prelude::*;

#[derive(Component)]
pub struct ExitTile;

#[derive(Component)]
pub struct FloorTile;

#[derive(Clone, Component, Copy)]
pub struct Player {
    pub id: usize,
}

#[derive(Clone, Component, Copy)]
pub struct MoveDir(pub Vec2);

#[derive(Component)]
pub struct WallTile;
