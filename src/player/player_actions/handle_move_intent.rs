use super::super::events::{PlayerAttacksEvent, PlayerMoveIntentEvent, PlayerMovesEvent};
use super::move_intent_handler::{MoveIntentHandler, ObstacleQuery, PlayerMove, PlayerQuery};
use bevy::prelude::*;

/// Dispatch an even based on the intended move (or possibly no event). An intended
/// move may be suppressed because it is throttled or because it is invalid (e.g.
/// trying to move into a wall).
pub fn handle_move_intent(
    mut attack_event: EventWriter<PlayerAttacksEvent>,
    mut event_reader: EventReader<PlayerMoveIntentEvent>,
    mut move_event: EventWriter<PlayerMovesEvent>,
    players: PlayerQuery,
    obstacles: ObstacleQuery,
) {
    event_reader
        .read()
        .map(|&event| MoveIntentHandler::new(event, &players))
        .filter(|h| !h.throttled)
        .filter_map(|h| h.determine_action(&obstacles))
        .for_each(|action| match action {
            PlayerMove::Attack(e) => {
                attack_event.send(e);
            }
            PlayerMove::Move(e) => {
                move_event.send(e);
            }
        });
}
