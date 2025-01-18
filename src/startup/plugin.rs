use super::events::DesyncEvent;
use super::startup::startup;
use crate::GameState;
use bevy::prelude::*;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DesyncEvent>()
            .add_systems(OnEnter(GameState::Startup), startup);
    }
}
