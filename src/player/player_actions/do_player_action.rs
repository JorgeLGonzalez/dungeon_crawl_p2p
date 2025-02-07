use super::*;
use crate::{
    common::SnapshotStateEvent,
    config::{self, GameMode},
    dungeon::{RevealDungeonEvent, ZoomEvent},
};
use bevy::prelude::*;
use bevy_ggrs::PlayerInputs;
use player_action::PickedItemQuery;

/// Determines the [`PlayerAction`] based on keyboard inputs and dispatches the
/// relevant event (e.g. [`PlayerMoveIntentEvent`]).
/// In GGRS modes, it handles the local and remote players based on [`PlayerInputs`].
/// In single-player mode, there's only one local player so inputs are read directly
/// from the Bevy [`ButtonInput`] resources
pub fn do_player_action(
    mut grab_event: EventWriter<GrabItemEvent>,
    mut move_event: EventWriter<PlayerMoveIntentEvent>,
    mut snapshot_event: EventWriter<SnapshotStateEvent>,
    mut reveal_event: EventWriter<RevealDungeonEvent>,
    mut stop_moving_event: EventWriter<StopMovingEvent>,
    mut use_item_event: EventWriter<UseItemEvent>,
    mut zoom_event: EventWriter<ZoomEvent>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    ggrs_inputs: Option<Res<PlayerInputs<config::GgrsSessionConfig>>>,
    picked_items: PickedItemQuery,
    players: Query<(Entity, &Player)>,
) {
    assert_player_count(players.iter().count());

    for (player_entity, player) in &players {
        let action = if let Some(ggrs_inputs) = ggrs_inputs.as_ref() {
            PlayerAction::from(ggrs_inputs[player.id].0)
        } else {
            PlayerAction::new(keys.as_mut(), &picked_items)
        };

        match action {
            PlayerAction::GrabItem => {
                grab_event.send(GrabItemEvent::new(player_entity, player.id));
            }
            PlayerAction::Move(dir) => {
                move_event.send(PlayerMoveIntentEvent::new(
                    player_entity,
                    player.id,
                    dir.to_ivec2(),
                ));
            }
            PlayerAction::None => (),
            PlayerAction::RevealDungeonCheat => {
                reveal_event.send(RevealDungeonEvent::new(player.id));
            }
            PlayerAction::Snapshot => {
                snapshot_event.send(SnapshotStateEvent::new(player.id));
            }
            PlayerAction::StopMoving => {
                stop_moving_event.send(StopMovingEvent::new(player_entity));
            }
            PlayerAction::UseItem(idx) => {
                use_item_event.send(UseItemEvent::new(player_entity, player.id, idx));
            }
            PlayerAction::ZoomIn => {
                zoom_event.send(ZoomEvent::zoom_in(player.id));
            }
            PlayerAction::ZoomOut => {
                zoom_event.send(ZoomEvent::zoom_out(player.id));
            }
        };
    }
}

fn assert_player_count(count: usize) {
    let expected_player_count = match config::GAME_MODE {
        GameMode::GgrsSyncTest => config::NUM_PLAYERS,
        GameMode::MultiPlayer => config::NUM_PLAYERS,
        GameMode::SinglePlayer => 1,
    };

    assert_eq!(count, expected_player_count, "Unexpected player count!");
}
