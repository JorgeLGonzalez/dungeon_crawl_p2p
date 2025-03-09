use super::{TooltipInfo, TooltipToggleTrigger, TooltipUIQuery};
use bevy::prelude::*;

pub fn on_exit_level(mut commands: Commands, tooltip_ui: TooltipUIQuery) {
    if TooltipInfo::entity(&tooltip_ui).is_some() {
        commands.trigger(TooltipToggleTrigger::Hide);
    }
}
