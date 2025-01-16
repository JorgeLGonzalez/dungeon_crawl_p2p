use super::super::{LocalPlayer, PlayersQuery};
use crate::player::{Player, PlayerCamera};
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

pub fn follow_with_camera(
    local_players: Res<LocalPlayers>,
    players: PlayersQuery,
    mut cameras: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
) {
    let player = LocalPlayer::new(&local_players, &players);

    let mut camera = cameras.single_mut();
    camera.translation.x = player.pos.x;
    camera.translation.y = player.pos.y;
}
