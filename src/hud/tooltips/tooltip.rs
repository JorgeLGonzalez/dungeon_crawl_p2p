use super::*;
use crate::player::PlayerMovesEvent;
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

/// Display or hide a tooltip when hovering over an entity marked with [`TooltipLabel`].
/// The [`TooltipUI`] component marks the entity used to display the tooltip whereas
/// the [`TooltipLabel`] component marks entities that can be hovered over to display
/// a tooltip.
/// Tooltips are only displayed for the local player.
/// We need to convert the mouse cursor position from window to world coordinates.
/// See README.md for more information.
pub fn tooltip(
    mut commands: Commands,
    mut cursor_events: EventReader<CursorMoved>,
    mut player_movement_events: EventReader<PlayerMovesEvent>,
    camera_query: CameraQuery,
    local_players: Res<LocalPlayers>,
    players: PlayerQuery,
    tooltip_entities: TooltipEntityQuery,
    tooltip_ui: TooltipUIQuery,
    windows: WindowQuery,
) {
    if let Some(toggle_action) = TooltipDeterminerBuilder::default()
        .mouse_info(&camera_query, &mut cursor_events, &windows)
        .local_player(&local_players, &players, &mut player_movement_events)
        .with_tooltip_ui(&tooltip_ui)
        .build()
        .determine(&tooltip_entities)
    {
        commands.trigger(toggle_action);
    }
}
