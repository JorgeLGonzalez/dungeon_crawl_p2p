use super::PlayerAction;
use crate::{
    components::Player,
    events::{PlayerMoveIntentEvent, SnapshotStateEvent, StopMovingEvent},
    resources::config::{self, GameMode},
};
use bevy::prelude::*;
use bevy_ggrs::PlayerInputs;

/// Determines the [`PlayerAction`] based on keyboard inputs and dispatches the
/// relevant event (e.g. [`PlayerMoveIntentEvent`]).
/// In GGRS modes, it handles the local and remote players based on [`PlayerInputs`].
/// In single-player mode, there's only one local player so inputs are read directly
/// from the Bevy [`ButtonInput`] resources
pub fn do_player_action(
    mut move_event: EventWriter<PlayerMoveIntentEvent>,
    mut snapshot_event: EventWriter<SnapshotStateEvent>,
    mut stop_moving_event: EventWriter<StopMovingEvent>,
    ggrs_inputs: Option<Res<PlayerInputs<config::GgrsSessionConfig>>>,
    keys: Res<ButtonInput<KeyCode>>,
    players: Query<(Entity, &Player)>,
) {
    assert_player_count(players.iter().count());

    for (player_entity, player) in &players {
        let action = if let Some(ggrs_inputs) = ggrs_inputs.as_ref() {
            PlayerAction::from(ggrs_inputs[player.id].0)
        } else {
            PlayerAction::from(keys.as_ref())
        };

        match action {
            PlayerAction::Move(dir) => {
                move_event.send(PlayerMoveIntentEvent::new(
                    player_entity,
                    player.id,
                    dir.to_vec2(),
                ));
            }
            PlayerAction::None => (),
            PlayerAction::Snapshot => {
                snapshot_event.send(SnapshotStateEvent::new(player.id));
            }
            PlayerAction::StopMoving => {
                stop_moving_event.send(StopMovingEvent::new(player_entity));
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
