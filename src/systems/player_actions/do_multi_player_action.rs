use crate::{
    components::Player,
    events::PlayerMoveIntentEvent,
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
    mut event_writer: EventWriter<PlayerMoveIntentEvent>,
    players: Query<(Entity, &Player)>,
    inputs: Res<PlayerInputs<GgrsSessionConfig>>,
) {
    assert_eq!(
        players.iter().count(),
        config::NUM_PLAYERS,
        "Unexpected player count!"
    );

    for (player_entity, player) in &players {
        if let Some(direction) = PlayerAction::from(inputs[player.id].0).move_direction() {
            event_writer.send(PlayerMoveIntentEvent::new(
                player_entity,
                player.id,
                direction,
            ));
        }
    }
}
