use bevy::{
    app::{App, Plugin},
    prelude::Event,
};

pub struct DungeonEventsPlugin;

impl Plugin for DungeonEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RevealDungeonEvent>()
            .add_event::<ZoomEvent>();
    }
}

/// Player used the magic dungeon map item or used the reveal map cheat
#[derive(Event)]
pub struct RevealDungeonEvent {
    /// ID of player who requested the reveal
    pub requestor_id: usize,
}

impl RevealDungeonEvent {
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
