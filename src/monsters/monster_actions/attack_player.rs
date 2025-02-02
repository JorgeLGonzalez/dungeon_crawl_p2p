use super::MonsterAttacksEvent;
use crate::prelude::*;

pub fn attack_player(
    mut attack_events: EventReader<MonsterAttacksEvent>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut players: Query<&mut Health, With<Player>>,
) {
    for event in attack_events.read() {
        let mut health = players.get_mut(event.player).expect("Inconceivable!");

        log(&health, event);

        if event.damage >= health.current {
            if !config::PLAYER_IMMORTAL {
                health.current = 0;
                next_state.set(GameState::GameOver);
            }
        } else {
            health.current -= event.damage;
            commands.entity(event.player).insert(Healing::default());
        }
    }
}

fn log(health: &Health, event: &MonsterAttacksEvent) {
    let remaining = health.current - event.damage.min(health.current);
    let action = if remaining > 0 { "attacks" } else { "kills" };
    let MonsterAttacksEvent {
        damage,
        monster,
        player_id,
        pos,
        ..
    } = event;
    info!(
        "Monster {monster} {action} player {player_id} at {pos} inflicting \
        {damage} damage points. Remaining health={remaining}/{}",
        health.max
    );
}
