use super::{queries::TooltipUIQuery, TooltipToggleTrigger};
use crate::{dungeon::ZoomEvent, player::LocalPlayer, prelude::*};
use bevy_ggrs::LocalPlayers;

/// Hide the tooltip when the local player zooms in or out.
pub fn on_zoom(
    mut commands: Commands,
    mut zoom_events: EventReader<ZoomEvent>,
    local_players: Res<LocalPlayers>,
    tooltip_ui: TooltipUIQuery,
) {
    let local_zoom = zoom_events
        .read()
        .last()
        .is_some_and(|e| LocalPlayer::is_local_player_id(e.requestor_id, &local_players));

    let (.., tooltip) = tooltip_ui.single();

    if tooltip.entity.is_some() && local_zoom {
        commands.trigger(TooltipToggleTrigger::Hide);
    };
}
