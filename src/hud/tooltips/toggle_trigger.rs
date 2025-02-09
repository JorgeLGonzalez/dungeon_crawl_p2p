use super::{MouseTooltip, PlayerTooltip, TooltipDisplayInfo, TooltipHider};
use bevy::prelude::*;

/// The event triggered from [`tooltip`] and observed by [`toggle_tooltip`].
#[derive(Event)]
pub enum TooltipToggleTrigger {
    /// Hide the tooltip using the provided hider.
    Hide(TooltipHider),
    ShowOnMouseCursor(TooltipDisplayInfo<MouseTooltip>),
    ShowOnPlayer(TooltipDisplayInfo<PlayerTooltip>),
}
