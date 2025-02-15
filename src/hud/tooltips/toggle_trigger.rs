mod mouse_position;
mod tooltip_info;
mod trigger_builder;

pub(super) use trigger_builder::TooltipToggleTriggerBuilder;

use super::queries::*;
use super::shower::*;
use super::TooltipLabel;
use bevy::prelude::*;
use mouse_position::MousePosition;
use tooltip_info::TooltipInfo;

/// The event triggered from tooltip systems monitoring mouse, player and monster
/// movement. Observed by [`toggle_tooltip`].
#[derive(Event, Debug)]
pub enum TooltipToggleTrigger {
    /// Hide the active tooltip
    Hide,
    /// Show a tooltip on the entity under the mouse cursor
    ShowOnMouseCursor(TooltipDisplayInfo<MouseTooltip>),
    /// Show a tooltip for the entity on which the player is standing
    ShowOnPlayer(TooltipDisplayInfo<PlayerTooltip>),
}
