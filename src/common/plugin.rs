use super::events::CommonEventsPlugin;
use bevy::prelude::*;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CommonEventsPlugin);
    }
}
