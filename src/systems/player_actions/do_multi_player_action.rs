use crate::{
    components::Player,
    events::PlayerMoveIntentEvent,
    resources::config::{self, GgrsSessionConfig},
    PlayerAction,
};
use bevy::prelude::*;
use bevy_ggrs::PlayerInputs;

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
        let action = PlayerAction::from(inputs[player.id].0);
        if let Some(direction) = determine_direction(action) {
            event_writer.send(PlayerMoveIntentEvent::new(
                player_entity,
                player.id,
                direction,
            ));
        }
    }
}

fn determine_direction(action: PlayerAction) -> Option<Vec2> {
    match action {
        PlayerAction::Up => Some(Vec2::Y),
        PlayerAction::Down => Some(Vec2::NEG_Y),
        PlayerAction::Left => Some(Vec2::NEG_X),
        PlayerAction::Right => Some(Vec2::X),
        _ => None,
    }
}
