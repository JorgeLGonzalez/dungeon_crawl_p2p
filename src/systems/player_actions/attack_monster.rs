use crate::PlayerAttackEvent;
use bevy::{log::info, prelude::*};

pub fn attack_monster(mut commands: Commands, mut event_reader: EventReader<PlayerAttackEvent>) {
    for event in event_reader.read() {
        let PlayerAttackEvent {
            monster,
            player_id,
            pos,
        } = event;
        info!("Player {player_id} attacks monster at {pos}");
        info!("Player {player_id} killed monster {monster:?}");
        commands.entity(event.monster).despawn_recursive();
    }
}
