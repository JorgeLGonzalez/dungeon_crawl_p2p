use super::PlayerAction;
use crate::components::Player;
use crate::PlayerMoveIntentEvent;
use bevy::prelude::*;

pub fn do_single_player_action(
    mut event_writer: EventWriter<PlayerMoveIntentEvent>,
    players: Query<(Entity, &Player)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    assert_eq!(players.iter().count(), 1, "Unexpected player count!");

    let (player_entity, player) = players.single();

    let action = PlayerAction::from(keys.as_ref());
    if let Some(direction) = determine_direction(action) {
        event_writer.send(PlayerMoveIntentEvent::new(
            player_entity,
            player.id,
            direction,
        ));
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
