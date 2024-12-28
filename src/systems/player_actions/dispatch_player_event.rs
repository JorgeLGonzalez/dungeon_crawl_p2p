use super::PlayerAction;
use crate::events::{PlayerMoveIntentEvent, SnapshotStateEvent, StopMovingEvent};
use bevy::prelude::{Entity, EventWriter};

pub fn dispatch_player_event(
    player: Entity,
    player_id: usize,
    action: PlayerAction,
    move_event: &mut EventWriter<PlayerMoveIntentEvent>,
    snapshot_event: &mut EventWriter<SnapshotStateEvent>,
    stop_moving_event: &mut EventWriter<StopMovingEvent>,
) {
    match action {
        PlayerAction::Move(dir) => {
            move_event.send(PlayerMoveIntentEvent::new(player, player_id, dir.to_vec2()));
        }
        PlayerAction::None => (),
        PlayerAction::Snapshot => {
            snapshot_event.send(SnapshotStateEvent::new(player_id));
        }
        PlayerAction::StopMoving => {
            stop_moving_event.send(StopMovingEvent::new(player));
        }
    }
}
