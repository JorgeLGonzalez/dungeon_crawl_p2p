use crate::{
    resources::{config, SessionSeed},
    GameState,
};
use bevy::{
    log::info,
    prelude::{Commands, NextState, ResMut},
};
use bevy_ggrs::ggrs::{self, DesyncDetection};

pub fn start_sync_test_session(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
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
    commands.insert_resource(SessionSeed::new());
    next_state.set(GameState::InGame);
}
