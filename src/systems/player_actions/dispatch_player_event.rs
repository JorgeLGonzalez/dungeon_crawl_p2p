use super::PlayerAction;
use crate::{PlayerMoveIntentEvent, StopMovingEvent};
use bevy::prelude::{Entity, EventWriter};

pub fn dispatch_player_event(
    player: Entity,
    player_id: usize,
    action: PlayerAction,
    move_event: &mut EventWriter<PlayerMoveIntentEvent>,
    stop_moving_event: &mut EventWriter<StopMovingEvent>,
) {
    match action {
        PlayerAction::Move(dir) => {
            move_event.send(PlayerMoveIntentEvent::new(player, player_id, dir.to_vec2()));
        }
        PlayerAction::None => (),
        PlayerAction::Snapshot => todo!(),
        PlayerAction::StopMoving => {
            stop_moving_event.send(StopMovingEvent::new(player));
        }
    }
}
