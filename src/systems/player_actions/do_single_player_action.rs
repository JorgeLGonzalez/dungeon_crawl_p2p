use super::dispatch_player_event::dispatch_player_event;
use super::PlayerAction;
use crate::components::Player;
use crate::{PlayerMoveIntentEvent, SnapshotStateEvent, StopMovingEvent};
use bevy::prelude::*;

/// Determines the `PlayerAction` based on keyboard inputs and dispatches the
/// relevant event (e.g. [`PlayerMoveIntentEvent`]). See [`super::do_multi_player_action`]
/// for the equivalent system for `GameMode::MultiPlayer`.
pub fn do_single_player_action(
    mut move_event: EventWriter<PlayerMoveIntentEvent>,
    mut snapshot_event: EventWriter<SnapshotStateEvent>,
    mut stop_moving_event: EventWriter<StopMovingEvent>,
    players: Query<(Entity, &Player)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    assert_eq!(players.iter().count(), 1, "Unexpected player count!");

    let (player_entity, player) = players.single();
    let action = PlayerAction::from(keys.as_ref());
    dispatch_player_event(
        player_entity,
        player.id,
        action,
        &mut move_event,
        &mut snapshot_event,
        &mut stop_moving_event,
    );
}
