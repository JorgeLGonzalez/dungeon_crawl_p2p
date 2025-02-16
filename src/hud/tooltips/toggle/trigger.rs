use super::*;
use bevy::prelude::*;

/// The event triggered from tooltip systems monitoring mouse, player and monster
/// movement. Observed by [`toggle_tooltip`].
#[derive(Event, Debug)]
pub enum TooltipToggleTrigger {
    /// Hide the active tooltip
    Hide,
    /// Show a tooltip on the entity under the mouse cursor or player
    Show(TooltipDisplayInfo),
}
