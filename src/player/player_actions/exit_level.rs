use super::PlayerMovesEvent;
use crate::{dungeon::ExitStairs, prelude::*};

pub fn exit_level(
    mut event_reader: EventReader<PlayerMovesEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    exit: Query<&Transform, With<ExitStairs>>,
) {
    let Some(event) = event_reader.read().last() else {
        return;
    };
    let exit_pos = exit.single().translation.truncate().as_ivec2();
    if event.pos != exit_pos {
        return;
    }

    info!("Exit level!");
    next_state.set(GameState::DungeonSpawning);
}
