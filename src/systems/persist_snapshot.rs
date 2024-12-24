use crate::{
    components::Player,
    resources::{
        config::GgrsSessionConfig, DesyncEvent, MonsterMove, MonsterMoveTracker, PlayerInputCode,
    },
};
use bevy::{
    log::{error, info},
    prelude::{EventReader, Query, Res, ResMut},
};
use bevy_ggrs::{LocalPlayers, PlayerInputs, RollbackFrameCount};
use std::{fs::OpenOptions, io::Write, path::Path};

pub fn persist_snapshot(
    mut event_reader: EventReader<DesyncEvent>,
    mut monster_tracker: ResMut<MonsterMoveTracker>,
    frame: Res<RollbackFrameCount>,
    inputs: Res<PlayerInputs<GgrsSessionConfig>>,
    local_player: Res<LocalPlayers>,
    players: Query<&Player>,
) {
    let Some(reason) = snapshot_reason(&mut event_reader, &inputs, &mut monster_tracker, &players)
    else {
        return;
    };

    let player_id = local_player.0[0];
    info!(
        "Taking snapshot for player {player_id} on frame {} because {reason:?}.",
        frame.0
    );

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
    monster_tracker.moves.clear();

    if matches!(reason, SnapshotReason::DesyncEvent) {
        file.flush().unwrap_or_else(|e| error!("{e}"));
        panic!("Aborting");
    }
}

fn snapshot_reason(
    event_reader: &mut EventReader<DesyncEvent>,
    inputs: &PlayerInputs<GgrsSessionConfig>,
    monster_tracker: &mut MonsterMoveTracker,
    players: &Query<&Player>,
) -> Option<SnapshotReason> {
    if monster_tracker.moves.len() >= 100 {
        Some(SnapshotReason::CountThreshold)
    } else if let Some(event) = event_reader.read().next() {
        info!(
            "Snapshot requested due to Desync event from frame {}",
            event.frame
        );
        Some(SnapshotReason::DesyncEvent)
    } else if players
        .iter()
        .filter_map(|player| PlayerInputCode::from_bits(inputs[player.id].0))
        .any(|input_code| matches!(input_code, PlayerInputCode::Snapshot))
    {
        Some(SnapshotReason::Requested)
    } else {
        None
    }
}

#[derive(Debug)]
enum SnapshotReason {
    CountThreshold,
    DesyncEvent,
    Requested,
}
