use super::PlayerAttacksEvent;
use crate::{
    health::{Healing, Health, HealthUnit},
    monsters::Monster,
};
use bevy::{log::info, prelude::*};

pub fn attack_monster(
    mut commands: Commands,
    mut event_reader: EventReader<PlayerAttacksEvent>,
    mut monsters: Query<&mut Health, With<Monster>>,
) {
    for event in event_reader.read() {
        let mut health = monsters.get_mut(event.monster).expect("Inconceivable!");

        log(event, health.current);

        if event.damage >= health.current {
            commands.entity(event.monster).despawn_recursive();
        } else {
            health.current -= event.damage;
            commands.entity(event.monster).insert(Healing::default());
        }
    }
}

fn log(event: &PlayerAttacksEvent, health: HealthUnit) {
    let PlayerAttacksEvent {
        damage,
        monster,
        player_id,
        pos,
    } = event;

    let revised_health = health.saturating_sub(*damage);

    info!(
        "Player {player_id} attacks monster {monster} at {pos} dealing \
    {damage} in damage, leaving monster with {revised_health} health.",
    );

    if revised_health <= 0 {
        info!("Monster {monster} dies!");
    }
}
