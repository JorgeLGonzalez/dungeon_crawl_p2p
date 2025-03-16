use super::{ExitStairs, FloorTile, WallTile};
use crate::dungeon::RespawnState;
use bevy::prelude::*;
use bevy_ggrs::RollbackFrameCount;

pub fn despawn_dungeon(
    mut commands: Commands,
    mut respawn: ResMut<RespawnState>,
    exit: Query<Entity, With<ExitStairs>>,
    floor: Query<Entity, With<FloorTile>>,
    frame: Res<RollbackFrameCount>,
    walls: Query<Entity, With<WallTile>>,
) {
    let frame = frame.0;

    info!("|HIGHLIGHT| Destroying current dungeon level (frame {frame})");
    floor
        .iter()
        .chain(walls.iter())
        .chain(exit.iter())
        .for_each(|e| commands.entity(e).despawn_recursive());

    *respawn = RespawnState::Init(frame);
}
