use bevy::{app::App, prelude::Event};

#[derive(Event)]
pub struct RevealDungeonCheatEvent {
    /// ID of player who requested the cheat
    pub requestor_id: usize,
}

impl RevealDungeonCheatEvent {
    pub fn new(requestor_id: usize) -> Self {
        Self { requestor_id }
    }
}

#[derive(Event)]
pub struct ZoomEvent {
    pub direction: ZoomDirection,
    pub requestor_id: usize,
}

impl ZoomEvent {
    pub fn zoom_in(requestor_id: usize) -> Self {
        Self {
            direction: ZoomDirection::In,
            requestor_id,
        }
    }

    pub fn zoom_out(requestor_id: usize) -> Self {
        Self {
            direction: ZoomDirection::Out,
            requestor_id,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ZoomDirection {
    In,
    Out,
}

pub fn add_events(app: &mut App) {
    app.add_event::<RevealDungeonCheatEvent>()
        .add_event::<ZoomEvent>();
}
