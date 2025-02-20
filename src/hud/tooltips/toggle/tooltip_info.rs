use super::*;
use bevy::prelude::*;

pub struct TooltipInfo;

impl TooltipInfo {
    /// Entity with active tooltip
    pub fn entity(tooltip_ui: &TooltipUIQuery) -> Option<Entity> {
        let (.., tooltip) = tooltip_ui.single();

        tooltip.entity
    }
}
