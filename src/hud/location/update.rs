use super::LocationText;
use crate::{
    player::{LocalPlayer, PlayerMovesEvent},
    prelude::*,
};
use bevy_ggrs::LocalPlayers;

pub fn update_location_ui(
    mut events: EventReader<PlayerMovesEvent>,
    mut query: Query<&mut Text, With<LocationText>>,
    local_players: Res<LocalPlayers>,
) {
    let Some(pos) = events
        .read()
        .filter(|e| LocalPlayer::is_local_player_id(e.player_id, &local_players))
        .last()
        .map(|e| e.pos)
    else {
        return;
    };

    query.single_mut().0 = format!("({},{})", pos.x, pos.y);
}
