use crate::{components::Player, events::PlayerMoveEvent, resources::config::PLAYER_Z_LAYER};
use bevy::{
    log::info,
    prelude::{EventReader, Query, Transform, With},
};

pub fn move_player(
    mut event_reader: EventReader<PlayerMoveEvent>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    for event in event_reader.read() {
        let mut transform = player.get_mut(event.player).expect("Player not found!");
        let old_pos = transform.translation.truncate();
        info!(
            "Player {} moves from {old_pos} to {}",
            event.player_id, event.pos
        );
        transform.translation = event.pos.extend(PLAYER_Z_LAYER);
    }
}
