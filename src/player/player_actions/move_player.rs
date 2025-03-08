use super::{MoveThrottle, Player, PlayerMovesEvent};
use crate::{config::PLAYER_Z_LAYER, fov::RecalculateFovEvent};
use bevy::prelude::*;

pub fn move_player(
    mut commands: Commands,
    mut event_reader: EventReader<PlayerMovesEvent>,
    mut player: Query<&mut Transform, With<Player>>,
    mut recalculate_fov: EventWriter<RecalculateFovEvent>,
) {
    for event in event_reader.read() {
        let mut transform = player.get_mut(event.player).expect("Player not found!");
        let old_pos = transform.translation.truncate();
        info!(
            "Player {} moves from {old_pos} to {}",
            event.player_id, event.pos
        );
        transform.translation = event.pos.as_vec2().extend(PLAYER_Z_LAYER);
        commands
            .entity(event.player)
            .insert(MoveThrottle::default());
        recalculate_fov.send(RecalculateFovEvent::new(event.player, event.pos));
    }
}
