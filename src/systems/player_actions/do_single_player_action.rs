use super::PlayerAction;
use crate::components::Player;
use crate::{PlayerMoveIntentEvent, StopMovingEvent};
use bevy::prelude::*;

/// Determines the `PlayerAction` based on keyboard inputs and dispatches the
/// relevant event (e.g. [`PlayerMoveIntentEvent`]). See [`super::do_multi_player_action`]
/// for the equivalent system for `GameMode::MultiPlayer`.
pub fn do_single_player_action(
    mut move_event: EventWriter<PlayerMoveIntentEvent>,
    mut stop_moving_event: EventWriter<StopMovingEvent>,
    players: Query<(Entity, &Player)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    assert_eq!(players.iter().count(), 1, "Unexpected player count!");

    let (player_entity, player) = players.single();
    let action = PlayerAction::from(keys.as_ref());

    if let Some(direction) = action.move_direction() {
        move_event.send(PlayerMoveIntentEvent::new(
            player_entity,
            player.id,
            direction,
        ));
    } else {
        match action {
            PlayerAction::StopMoving => {
                stop_moving_event.send(StopMovingEvent::new(player_entity));
            }
            PlayerAction::Snapshot => todo!(),
            PlayerAction::None => (),
            _ => unreachable!(),
        }
    }
}
