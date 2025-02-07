use super::{FloorTile, RevealDungeonEvent};
use crate::player::LocalPlayer;
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

/// Reveal the map to player who used the magic map item or to both players if
/// one of them used the reveal map cheat
pub fn reveal_map(
    mut reveal_events: EventReader<RevealDungeonEvent>,
    mut tiles: Query<&mut Visibility, With<FloorTile>>,
    local_players: Res<LocalPlayers>,
) {
    let Some(event) = reveal_events.read().next() else {
        return;
    };

    if !event.cheat && !LocalPlayer::is_local_player_id(event.requestor_id, &local_players) {
        return;
    }

    info!("Reveal cheat requested by player {}", event.requestor_id);

    tiles
        .iter_mut()
        .filter(|v| **v == Visibility::Hidden)
        .for_each(|mut v| {
            *v = Visibility::Visible;
        });

    reveal_events.clear();
}
