use crate::events::PlayerAttacksEvent;
use bevy::{log::info, prelude::*};

pub fn attack_monster(mut commands: Commands, mut event_reader: EventReader<PlayerAttacksEvent>) {
    for event in event_reader.read() {
        let PlayerAttacksEvent {
            damage,
            monster,
            player_id,
            pos,
        } = event;
        info!("Player {player_id} attacks monster at {pos} dealing {damage} damage");
        info!("Player {player_id} killed monster {monster:?}");
        commands.entity(event.monster).despawn_recursive();
    }
}
