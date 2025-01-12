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

pub fn add_events(app: &mut App) {
    app.add_event::<RevealDungeonCheatEvent>();
}
