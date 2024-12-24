use crate::{
    components::Player,
    resources::{config::GgrsSessionConfig, MonsterMove, MonsterMoveTracker, PlayerInputCode},
};
use bevy::{
    core::FrameCount,
    log::{error, info},
    prelude::{Query, Res},
};
use bevy_ggrs::{ConfirmedFrameCount, LocalPlayers, PlayerInputs, RollbackFrameCount, Session};
use std::{cmp::Ordering, fs::OpenOptions, io::Write, path::Path};

pub fn persist_snapshot(
    inputs: Res<PlayerInputs<GgrsSessionConfig>>,
    local_player: Res<LocalPlayers>,
    monster_tracker: Res<MonsterMoveTracker>,
    players: Query<&Player>,
    // session: Res<Session<GgrsSessionConfig>>,
) {
    let snapshot_requested = players
        .iter()
        .filter_map(|player| PlayerInputCode::from_bits(inputs[player.id].0))
        .any(|input_code| matches!(input_code, PlayerInputCode::Snapshot));

    if !snapshot_requested {
        return;
    }

    let player_id = local_player.0[0];
    info!("Taking snapshot for player {player_id}.");

    let mut moves: Vec<&MonsterMove> = monster_tracker.moves.iter().collect::<Vec<_>>();
    moves.sort_by(|a, b| {
        a.monster
            .cmp(&b.monster)
            .then_with(|| a.frame.cmp(&b.frame))
    });

    let file_name = format!("{player_id}_monster_moves.csv");
    let exists = Path::try_exists(Path::new(&file_name)).unwrap_or_default();
    let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_name)
    else {
        return error!("Error creating file {file_name}");
    };

    if !exists {
        writeln!(file, "{}", MonsterMove::csv_headings()).unwrap_or_else(|e| error!("{e}"));
    }

    for movement in &moves {
        writeln!(file, "{}", movement.to_csv()).unwrap_or_else(|e| error!("{e}"));
    }

    info!("Saved {} monster moves to {}", moves.len(), file_name);

    // TODO throttle snapshots and only take for local player. Act on release, actually
}
