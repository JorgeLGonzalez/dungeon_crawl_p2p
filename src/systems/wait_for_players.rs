use crate::{
    resources::{config, MatchboxSocketResource, SessionSeed},
    GameState,
};
use bevy::{
    log::info,
    prelude::{Commands, NextState, ResMut},
};
use bevy_ggrs::ggrs;

pub fn wait_for_players(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut socket_resource: ResMut<MatchboxSocketResource>,
) {
    let socket = &mut socket_resource.0;
    if socket.get_channel(0).is_err() {
        return; // we've already started
    }

    socket.update_peers();
    let players = socket.players();
    if players.len() < config::NUM_PLAYERS {
        return;
    }

    info!("All peers have joined. Starting game!");
    let id = socket.id().expect("No peer ID!").0.as_u64_pair();
    let mut seed = id.0 ^ id.1;
    for peer in socket.connected_peers() {
        let peer_id = peer.0.as_u64_pair();
        seed ^= peer_id.0 ^ peer_id.1;
    }
    commands.insert_resource(SessionSeed(seed));

    let mut session_builder = ggrs::SessionBuilder::<config::GgrsSessionConfig>::new()
        .with_num_players(config::NUM_PLAYERS)
        .with_input_delay(2);
    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("Failed to add player to session");
    }

    let channel = socket.take_channel(0).unwrap();

    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("Failed to start session");

    commands.insert_resource(bevy_ggrs::Session::P2P(ggrs_session));

    next_state.set(GameState::InGame);
}
