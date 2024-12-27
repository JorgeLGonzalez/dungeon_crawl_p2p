use super::move_intent_handler::{MoveIntentHandler, ObstacleQuery, PlayerMove};
use crate::{
    components::{Player, PlayerMovement},
    events::{PlayerAttackEvent, PlayerMoveEvent, PlayerMoveIntentEvent},
};
use bevy::prelude::*;

/// Dispatch an even based on the intended move (or possibly no event). An intended
/// move may be suppressed because it is throttled or because it is invalid (e.g.
/// trying to move into a wall).
pub fn handle_move_intent(
    mut attack_event: EventWriter<PlayerAttackEvent>,
    mut event_reader: EventReader<PlayerMoveIntentEvent>,
    mut move_event: EventWriter<PlayerMoveEvent>,
    mut player_info: Query<(&mut PlayerMovement, &Transform), With<Player>>,
    obstacles: ObstacleQuery,
    time: Res<Time>,
) {
    event_reader.read().for_each(|&event| {
        let (mut prior_movement, transform) = player_info
            .get_mut(event.player)
            .expect("Player not found!");

        let action = MoveIntentHandler::new(event, transform)
            .update_movement(&mut prior_movement, time.delta())
            .determine_action(&obstacles);

        if let Some(action) = action {
            match action {
                PlayerMove::Attack(e) => {
                    attack_event.send(e);
                }
                PlayerMove::Move(e) => {
                    move_event.send(e);
                }
            }
        }
    });
}
