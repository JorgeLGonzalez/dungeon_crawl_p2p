use crate::{
    dungeon::{ExitStairs, RespawnState},
    player::events::ExitLevelEvent,
    prelude::*,
};
use bevy_ggrs::RollbackFrameCount;

pub fn exit_level(
    mut event_reader: EventReader<ExitLevelEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut respawn: ResMut<RespawnState>,
    exit: Query<&Transform, With<ExitStairs>>,
    frame: Res<RollbackFrameCount>,
    player: Query<&Transform, With<Player>>,
) {
    let Some(event) = event_reader.read().last() else {
        return;
    };
    let exit_pos = exit.single().translation.truncate().as_ivec2();

    let player_pos = player
        .get(event.player)
        .expect("Inconceivable!")
        .translation
        .truncate()
        .as_ivec2();
    if player_pos != exit_pos {
        return;
    }

    let frame = frame.0;

    info!(
        "|HIGHLIGHT| Player {} takes exit stairs at {}, exiting level! (frame {frame})",
        event.player_id, exit_pos
    );
    next_state.set(GameState::DungeonSpawning);
    *respawn = RespawnState::Pending(frame);
}
