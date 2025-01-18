use crate::{
    common::{DesyncEvent, RandomGenerator},
    config::GgrsSessionConfig,
    monsters::Monster,
    player::{MoveThrottle, Player},
    GameState,
};
use bevy::prelude::*;
use bevy_ggrs::{
    ggrs::GgrsEvent, GgrsComponentSnapshots, GgrsResourceSnapshots, GgrsSnapshots, LocalPlayers,
    Session,
};
use std::fmt::Debug;
use std::{fs::OpenOptions, io::Write};

pub fn handle_ggrs_events(
    mut event_writer: EventWriter<DesyncEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut session: ResMut<Session<GgrsSessionConfig>>,
    local_players: Res<LocalPlayers>,
    monster_snapshots: Res<GgrsComponentSnapshots<Monster>>,
    player_movement_snapshots: Res<GgrsComponentSnapshots<MoveThrottle>>,
    player_snapshots: Res<GgrsComponentSnapshots<Player>>,
    rng_snapshots: Res<GgrsResourceSnapshots<RandomGenerator>>,
    transform_snapshots: Res<GgrsComponentSnapshots<Transform>>,
) {
    match session.as_mut() {
        Session::P2P(s) => {
            for event in s.events() {
                match event {
                    GgrsEvent::Disconnected { .. } | GgrsEvent::NetworkInterrupted { .. } => {
                        warn!("GGRS event: {event:?}")
                    }
                    GgrsEvent::DesyncDetected {
                        frame,
                        local_checksum,
                        remote_checksum,
                        ..
                    } => {
                        let player_id = local_players.0[0];
                        error!(
                            "GGRS event: Desync on frame {frame} player {player_id}. \
                         Local checksum: {local_checksum:X}, remote checksum: {remote_checksum:X}"
                        );
                        // Note the below is not useful unless bevy_ggrs keeps enough snapshots around
                        // See [issue](https://github.com/gschup/bevy_ggrs/issues/117)
                        log_component_snapshot(&monster_snapshots, frame);
                        log_component_snapshot(&player_movement_snapshots, frame);
                        let player_entity_info = log_component_snapshot(&player_snapshots, frame);
                        log_res_snapshot(&rng_snapshots, frame);
                        log_component_snapshot(&transform_snapshots, frame);

                        log_to_file(&transform_snapshots, frame, player_id, &player_entity_info);

                        info!("Pausing game. Press P to take a snapshot of monster moves.");
                        event_writer.send(DesyncEvent { frame });
                        next_state.set(GameState::Paused);
                    }

                    _ => info!("GGRS event: {event:?}"),
                }
            }
        }
        // TODO: this depends on my PR being merged https://github.com/gschup/ggrs/pull/98
        // Session::SyncTest(s) => {
        //     info!("handle_ggrs_events: frame {}", s.current_frame());
        //     for event in s.events() {
        //         match event {
        //             GgrsEvent::MismatchedChecksum {
        //                 current_frame,
        //                 mismatched_frame: frame,
        //                 ..
        //             } => {
        //                 let player_id = local_players.0[0];
        //                 error!(
        //                     "GGRSEvent::MismatchedChecksum: Detected checksum mismatch during rollback \
        //                      on frame {current_frame}, oldest mismatched frame: {frame}. Player={player_id}"
        //                 );
        //                 log_component_snapshot(&monster_snapshots, frame);
        //                 log_component_snapshot(&player_movement_snapshots, frame);
        //                 log_component_snapshot(&player_snapshots, frame);
        //                 log_res_snapshot(&rng_snapshots, frame);
        //                 log_component_snapshot(&transform_snapshots, frame);

        //                 assert_eq!(player_id, 0);
        //             }
        //             _ => info!("GGRS event: {event:?}"),
        //         }
        //     }
        // }
        _ => (),
    }
}

fn log_to_file(
    container: &GgrsComponentSnapshots<Transform>,
    frame: i32,
    player_id: usize,
    player_entity_info: &str,
) {
    let Some(snapshots) = container.peek(frame) else {
        return;
    };

    let mut rows = snapshots
        .iter()
        .map(|(r, t)| format!("{r:?},{},{}", t.translation.x, t.translation.y))
        .collect::<Vec<_>>();
    rows.sort();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("{player_id}_transforms.csv"))
        .expect("Unable to create file");

    writeln!(
        file,
        "Frame,{frame},player,{player_id},player_entity,{player_entity_info}"
    )
    .expect("error writing row");

    rows.iter().for_each(|r| {
        writeln!(file, "{r}").expect("error writing row");
    });
}

fn log_component_snapshot<T: Debug>(container: &GgrsComponentSnapshots<T>, frame: i32) -> String {
    let name = get_name::<T>();
    let Some(snapshots) = container.peek(frame) else {
        handle_unavailable_snapshot(&name, container, frame, |s| s.iter().count());
        return String::new();
    };

    info!(
        "Frame {frame} {name} snapshots {}",
        snapshots.iter().count()
    );
    snapshots
        .iter()
        .fold(String::new(), |acc, (rollback, component)| {
            info!("{name} snapshot: {component:?} [{rollback:?}]");
            match acc {
                acc if !acc.is_empty() => acc,
                // return the entity id within the string of format "Rollback(5238v1#4294972534)"
                _ => format!("{rollback:?}")[9..26].to_string(),
            }
        })
}

fn log_res_snapshot<T: std::fmt::Debug>(container: &GgrsResourceSnapshots<T>, frame: i32) {
    let name = get_name::<T>();
    let Some(Some(snapshot)) = container.peek(frame) else {
        return handle_unavailable_snapshot(&name, container, frame, |_| 1);
    };

    info!("Frame {frame} {name} resource {:?}", snapshot);
}

fn get_name<T>() -> String {
    std::any::type_name::<T>()
        .split("::")
        .last()
        .unwrap_or("(Unknown)")
        .to_string()
}

fn handle_unavailable_snapshot<T, S, U>(
    name: &str,
    _container: &GgrsSnapshots<T, S>,
    frame: i32,
    _get_count: U,
) where
    T: Debug,
    U: Fn(&S) -> usize,
{
    warn!("Desync frame {frame} unavailable in {name} snapshot history");
    // TODO below require bevy_ggrs P2PSession::frames to be public
    // info!(
    //     "{name} history contains only {} frames as follows:",
    //     container.frames.len()
    // );
    // container.frames.iter().for_each(|s_frame| {
    //     let snapshot_count = container
    //         .peek(*s_frame)
    //         .map(|snapshot| get_count(snapshot))
    //         .unwrap_or_default();
    //     info!("\tFrame {s_frame} with {snapshot_count} snapshots");
    // });
}
