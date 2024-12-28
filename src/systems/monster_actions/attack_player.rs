use crate::{GameState, MonsterAttacksEvent};
use bevy::prelude::*;

pub fn attack_player(
    mut attack_events: EventReader<MonsterAttacksEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in attack_events.read() {
        info!(
            "Monster {} attacks player {} at {}",
            event.monster, event.player_id, event.pos
        );
        next_state.set(GameState::GameOver);
    }
}
