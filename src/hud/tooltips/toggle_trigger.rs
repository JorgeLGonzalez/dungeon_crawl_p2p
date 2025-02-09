use super::{MouseTooltip, PlayerTooltip, TooltipDisplayInfo};
use bevy::prelude::*;

/// The event triggered from [`tooltip`] and observed by [`toggle_tooltip`].
#[derive(Event)]
pub enum TooltipToggleTrigger {
    /// Hide the active tooltip
    Hide,
    /// Show a tooltip on the entity under the mouse cursor
    ShowOnMouseCursor(TooltipDisplayInfo<MouseTooltip>),
    /// Show a tooltip for the entity on which the player is standing
    ShowOnPlayer(TooltipDisplayInfo<PlayerTooltip>),
}
