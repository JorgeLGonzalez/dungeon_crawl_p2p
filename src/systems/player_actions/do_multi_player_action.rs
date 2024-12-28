use super::dispatch_player_event::dispatch_player_event;
use crate::{
    components::Player,
    events::{PlayerMoveIntentEvent, SnapshotStateEvent, StopMovingEvent},
    resources::config::{self, GgrsSessionConfig},
    PlayerAction,
};
use bevy::prelude::*;
use bevy_ggrs::PlayerInputs;

/// Handles the [`PlayerAction`] decoded from [`PlayerInputs`] by dispatching
/// the relevant event (e.g. [`PlayerMoveIntentEvent`]).
/// See [`super::do_single_player_action`] for the equivalent system for
/// `GameMode::SinglePlayer`.
pub fn do_multi_player_action(
    mut move_event: EventWriter<PlayerMoveIntentEvent>,
    mut snapshot_event: EventWriter<SnapshotStateEvent>,
    mut stop_moving_event: EventWriter<StopMovingEvent>,
    players: Query<(Entity, &Player)>,
    inputs: Res<PlayerInputs<GgrsSessionConfig>>,
) {
    assert_eq!(
        players.iter().count(),
        config::NUM_PLAYERS,
        "Unexpected player count!"
    );

    for (player_entity, player) in &players {
        let action = PlayerAction::from(inputs[player.id].0);
        dispatch_player_event(
            player_entity,
            player.id,
            action,
            &mut move_event,
            &mut snapshot_event,
            &mut stop_moving_event,
        );
    }
}
