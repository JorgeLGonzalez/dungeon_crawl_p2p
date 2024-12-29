use crate::{
    components::{Health, Player},
    events::MonsterAttacksEvent,
    GameState,
};
use bevy::prelude::*;

pub fn attack_player(
    mut attack_events: EventReader<MonsterAttacksEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut players: Query<&mut Health, With<Player>>,
) {
    for event in attack_events.read() {
        let mut health = players.get_mut(event.player).expect("Inconceivable!");
        health.current -= 1;
        log(&health, event);

        if health.current == 0 {
            next_state.set(GameState::GameOver);
        }
    }
}

fn log(health: &Health, event: &MonsterAttacksEvent) {
    let result = if health.current > 0 {
        "attacks"
    } else {
        "kills"
    };
    let MonsterAttacksEvent {
        monster,
        player_id,
        pos,
        ..
    } = event;
    info!(
        "Monster {monster} {result} player {player_id} at {pos}. Current health={}/{}",
        health.current, health.max
    );
}
