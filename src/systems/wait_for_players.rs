use bevy::{log::info, prelude::ResMut};

use crate::resources::MatchboxSocketResource;

pub fn wait_for_players(mut socket_resource: ResMut<MatchboxSocketResource>) {
    let socket = &mut socket_resource.0;
    if socket.get_channel(0).is_err() {
        return; // we've already started
    }

    socket.update_peers();
    let players = socket.players();
    if players.len() < 2 {
        return;
    }

    info!("All peers have joined. Starting game!");
}
