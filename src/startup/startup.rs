use crate::{
    common::RandomGenerator,
    config::{self, GameMode},
    GameState,
};
use bevy::prelude::*;
use bevy_ggrs::ggrs::{self, DesyncDetection};
use bevy_matchbox::{
    prelude::{ChannelConfig, WebRtcSocketBuilder},
    MatchboxSocket,
};

pub fn startup(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    match config::GAME_MODE {
        GameMode::GgrsSyncTest => start_sync_test_session(&mut commands, &mut next_state),
        GameMode::MultiPlayer => connect_to_matchbox(&mut commands),
        GameMode::SinglePlayer => start_single_player_mode(&mut commands, &mut next_state),
    }
}

fn connect_to_matchbox(commands: &mut Commands) {
    let room_url = config::MATCHBOX_ROOM_URL;
    info!("Connecting to matchbox server {room_url}");
    let socket: MatchboxSocket = WebRtcSocketBuilder::new(room_url)
        .add_channel(ChannelConfig::unreliable())
        .into();
    commands.insert_resource(socket);
}

fn start_single_player_mode(commands: &mut Commands, next_state: &mut NextState<GameState>) {
    info!("Starting single player game.");
    commands.insert_resource(RandomGenerator::new());
    next_state.set(GameState::InGame);
}

fn start_sync_test_session(commands: &mut Commands, next_state: &mut NextState<GameState>) {
    info!("Starting sync-test session");
    let mut session_builder = ggrs::SessionBuilder::<config::GgrsSessionConfig>::new()
        .with_num_players(config::NUM_PLAYERS)
        .with_desync_detection_mode(DesyncDetection::On { interval: 1 });

    for i in 0..config::NUM_PLAYERS {
        session_builder = session_builder
            .add_player(ggrs::PlayerType::Local, i)
            .expect("Failed to add player");
    }

    let ggrs_session = session_builder
        .start_synctest_session()
        .expect("Failed to start session");

    commands.insert_resource(bevy_ggrs::Session::SyncTest(ggrs_session));
    commands.insert_resource(RandomGenerator::new());
    next_state.set(GameState::InGame);
}
