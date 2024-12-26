use super::PlayerAction;
use crate::components::Player;
use crate::PlayerMoveIntentEvent;
use bevy::prelude::*;

/// Determines the `PlayerAction` based on keyboard inputs and dispatches the
/// relevant event (e.g. [`PlayerMoveIntentEvent`]). See [`super::do_multi_player_action`]
/// for the equivalent system for `GameMode::MultiPlayer`.
pub fn do_single_player_action(
    mut event_writer: EventWriter<PlayerMoveIntentEvent>,
    players: Query<(Entity, &Player)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    assert_eq!(players.iter().count(), 1, "Unexpected player count!");

    let (player_entity, player) = players.single();

    if let Some(direction) = PlayerAction::from(keys.as_ref()).move_direction() {
        event_writer.send(PlayerMoveIntentEvent::new(
            player_entity,
            player.id,
            direction,
        ));
    }
}
