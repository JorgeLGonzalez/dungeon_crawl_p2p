use crate::{
    components::{Monster, Player},
    resources::{config::GgrsSessionConfig, RandomGenerator},
};
use bevy::{
    log::{error, info, warn},
    prelude::{Res, ResMut, Transform},
};
use bevy_ggrs::{
    ggrs::GgrsEvent, GgrsComponentSnapshots, GgrsResourceSnapshots, GgrsSnapshots, Session,
};
use std::fmt::Debug;

pub fn handle_ggrs_events(
    mut session: ResMut<Session<GgrsSessionConfig>>,
    monster_snapshots: Res<GgrsComponentSnapshots<Monster>>,
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
                        error!(
                            "GGRS event: Desync on frame {frame}. \
                         Local checksum: {local_checksum:X}, remote checksum: {remote_checksum:X}"
                        );
                        log_component_snapshot(&monster_snapshots, frame);
                        log_component_snapshot(&player_snapshots, frame);
                        log_res_snapshot(&rng_snapshots, frame);
                        log_component_snapshot(&transform_snapshots, frame);

                        panic!("Desync!");
                    }

                    _ => info!("GGRS event: {event:?}"),
                }
            }
        }

        _ => (),
    }
}

fn log_component_snapshot<T: Debug>(container: &GgrsComponentSnapshots<T>, frame: i32) {
    let name = get_name::<T>();
    let Some(snapshots) = container.peek(frame) else {
        return handle_unavailable_snapshot(&name, container, frame, |s| s.iter().count());
    };

    info!(
        "Frame {frame} {name} snapshots {}",
        snapshots.iter().count()
    );
    snapshots.iter().for_each(|(_rollback, component)| {
        info!("{name} snapshot: {component:?}");
    });
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

fn handle_unavailable_snapshot<T: Debug, S, U>(
    name: &str,
    container: &GgrsSnapshots<T, S>,
    frame: i32,
    get_count: U,
) where
    T: Debug,
    U: Fn(&S) -> usize,
{
    warn!("Desync frame {frame} unavailable in {name} snapshot history");
    info!(
        "{name} history contains only {} frames as follows:",
        container.frames.len()
    );
    container.frames.iter().for_each(|s_frame| {
        let snapshot_count = container
            .peek(*s_frame)
            .map(|snapshot| get_count(snapshot))
            .unwrap_or_default();
        info!("\tFrame {s_frame} with {snapshot_count} snapshots");
    });
}
