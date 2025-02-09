use super::*;
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
    camera_query: CameraQuery,
    local_players: Res<LocalPlayers>,
    players: PlayerQuery,
    tooltip_entities: TooltipEntityQuery,
    tooltip_ui: TooltipUIQuery,
    windows: WindowQuery,
) {
    if TooltipDeterminerBuilder::new(&tooltip_ui)
        .mouse_info(&camera_query, &windows)
        .add_player_info(&local_players, &players)
        .build()
        .should_hide(&tooltip_entities)
    {
        commands.trigger(TooltipToggleTrigger::Hide);
    }
}
