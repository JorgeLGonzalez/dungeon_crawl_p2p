use crate::PlayerAttackEvent;
use bevy::{log::info, prelude::*};

pub fn attack_monster(mut commands: Commands, mut event_reader: EventReader<PlayerAttackEvent>) {
    for event in event_reader.read() {
        info!(
            "Player {} killed monster {:?}",
            event.player_id, event.monster
        );
        commands.entity(event.monster).despawn_recursive();
    }
}
