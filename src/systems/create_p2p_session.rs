use crate::{
    resources::{
        config::{self, GgrsSessionConfig},
        MatchboxSocketResource, RandomGenerator,
    },
    GameState,
};
use bevy::{
    log::info,
    prelude::{Commands, NextState, ResMut},
};
use bevy_ggrs::{
    ggrs::{self, DesyncDetection, PlayerType},
    Session,
};
use bevy_matchbox::{
    prelude::{PeerId, SingleChannel},
    MatchboxSocket,
};

pub fn create_p2p_session(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut socket_resource: ResMut<MatchboxSocketResource>,
) {
    let socket = &mut socket_resource.0;
    if socket.get_channel(0).is_err() {
        return; // we've already started so ggrs has taken the socket channel
    }

    socket.update_peers();
    let players = socket.players();
    if players.len() < config::NUM_PLAYERS {
        return;
    }

    info!("All peers have joined. Starting game!");
    commands.insert_resource(RandomGenerator::new_for_p2p(socket));
    commands.insert_resource(build_session(players, socket));

    next_state.set(GameState::InGame);
}

fn build_session(
    players: Vec<PlayerType<PeerId>>,
    socket: &mut MatchboxSocket<SingleChannel>,
) -> Session<GgrsSessionConfig> {
    let mut session_builder = ggrs::SessionBuilder::<config::GgrsSessionConfig>::new()
        .with_num_players(config::NUM_PLAYERS)
        .with_desync_detection_mode(DesyncDetection::On { interval: 1 })
        .with_input_delay(config::GGRS_INPUT_DELAY);
    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("Failed to add player to session");
    }

    let channel = socket.take_channel(0).unwrap();

    bevy_ggrs::Session::P2P(
        session_builder
            .start_p2p_session(channel)
            .expect("Failed to start session"),
    )
}
