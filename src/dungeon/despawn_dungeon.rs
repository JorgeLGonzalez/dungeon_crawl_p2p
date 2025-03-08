use super::{ExitStairs, FloorTile, WallTile};
use bevy::prelude::*;

pub fn despawn_dungeon(
  mut commands: Commands,
  exit: Query<Entity, With<ExitStairs>>,
  floor: Query<Entity, With<FloorTile>>,
  walls: Query<Entity, With<WallTile>>,
) {
  floor
      .iter()
      .chain(walls.iter())
      .chain(exit.iter())
      .for_each(|e| commands.entity(e).despawn_recursive());
}
