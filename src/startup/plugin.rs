use super::events::DesyncEvent;
use bevy::prelude::*;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DesyncEvent>();
    }
}
