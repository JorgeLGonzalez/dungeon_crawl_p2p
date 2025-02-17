use super::{queries::TooltipUIQuery, TooltipInfo, TooltipToggleTrigger};
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

    let is_tooltip_active = TooltipInfo::entity(&tooltip_ui).is_some();

    if is_tooltip_active && local_zoom {
        commands.trigger(TooltipToggleTrigger::Hide);
    };
}
