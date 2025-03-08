use super::PlayerMovesEvent;
use crate::{dungeon::ExitStairs, prelude::*};

pub fn exit_level(
    // mut commands: Commands,
    mut event_reader: EventReader<PlayerMovesEvent>,
    exit: Query<&Transform, With<ExitStairs>>,
) {
    let Some(event) = event_reader.read().last() else {
        return;
    };
    let exit_pos = exit.single().translation.truncate().as_ivec2();

    if event.pos == exit_pos {
        info!("Exit level!");
    }
}
