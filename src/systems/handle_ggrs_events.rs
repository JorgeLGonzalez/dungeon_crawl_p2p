use crate::resources::config::GgrsSessionConfig;
use bevy::{
    log::{error, info, warn},
    prelude::ResMut,
};
use bevy_ggrs::{ggrs::GgrsEvent, Session};

pub fn handle_ggrs_events(mut session: ResMut<Session<GgrsSessionConfig>>) {
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
                    } => error!(
                        "GGRS event: Desync on frame {frame}.\
                         Local checksum: {local_checksum:X}, remote checksum: {remote_checksum:X}"
                    ),
                    _ => info!("GGRS event: {event:?}"),
                }
            }
        }

        _ => (),
    }
}
