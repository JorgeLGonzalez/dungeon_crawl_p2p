pub mod assets;
pub mod config;

mod events;
mod plugin;
mod startup;

mod ggrs {
    mod checksum_transform;
    mod create_p2p_session;
    mod handle_ggrs_events;

    pub(super) use checksum_transform::checksum_transform;
    pub(super) use create_p2p_session::create_p2p_session;
    pub(super) use handle_ggrs_events::handle_ggrs_events;
}

pub use events::DesyncEvent;
pub use plugin::StartupPlugin;
