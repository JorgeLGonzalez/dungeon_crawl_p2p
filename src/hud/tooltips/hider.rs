use super::queries::TooltipUIQuery;
use bevy::prelude::*;

pub struct TooltipHider;

impl TooltipHider {
    /// Hides the tooltip and disassociates it from the entity.
    pub fn hide(&self, tooltip_ui: &mut TooltipUIQuery) {
        let (mut tooltip_node, mut tooltip_text, mut tooltip) = tooltip_ui.single_mut();
        tooltip_node.display = Display::None;
        tooltip_text.0 = String::new();
        tooltip.entity = None;
    }
}
