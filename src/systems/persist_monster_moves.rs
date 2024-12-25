use super::PlayerAction;
use crate::{
    components::Player,
    resources::{config::GgrsSessionConfig, DesyncEvent, MonsterMove, MonsterMoveTracker},
};
use bevy::{
    log::{error, info},
    prelude::{EventReader, Query, Res, ResMut},
};
use bevy_ggrs::{LocalPlayers, PlayerInputs, RollbackFrameCount};
use std::{fs::OpenOptions, io::Write, path::Path};

/// Save monster moves to a file. (Won't work on WASM).
/// Three reasons why a save can take place:
/// 1. Requested by a player pressing and releasing P
/// 2. A desync event was written by [`super::handle_ggrs_events`]
/// 3. Autosave is enabled and reached its threshold
pub fn persist_monster_moves(
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
    if monster_tracker.threshold() {
        Some(SnapshotReason::CountThreshold)
    } else if let Some(event) = event_reader.read().next() {
        info!(
            "Snapshot requested due to Desync event from frame {}",
            event.frame
        );
        Some(SnapshotReason::DesyncEvent)
    } else {
        players
            .iter()
            .map(|player| PlayerAction::from(inputs[player.id].0))
            .find(|&action| action == PlayerAction::Snapshot)
            .map(|_| SnapshotReason::Requested)
    }
}

#[derive(Debug)]
enum SnapshotReason {
    CountThreshold,
    DesyncEvent,
    Requested,
}
