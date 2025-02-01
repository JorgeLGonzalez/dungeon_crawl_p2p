use super::{
    determiner::TooltipToggleAction, determiner_builder::TooltipDeterminerBuilder, queries::*,
};
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
    mut cursor_events: EventReader<CursorMoved>,
    mut tooltip_ui: TooltipUIQuery,
    camera_query: CameraQuery,
    hud_camera_query: HudCameraQuery,
    local_players: Res<LocalPlayers>,
    players: PlayerQuery,
    tooltip_entities: TooltipEntityQuery,
    windows: WindowQuery,
) {
    let toggle_action = TooltipDeterminerBuilder::new(&camera_query, &mut cursor_events, &windows)
        .local_player_fov(&local_players, &players)
        .with_tooltip_ui(&mut tooltip_ui)
        .build()
        .determine(&tooltip_entities);

    match toggle_action {
        TooltipToggleAction::Hide(hider) => hider.hide(&mut tooltip_ui),
        TooltipToggleAction::None => {}
        TooltipToggleAction::Show(shower) => shower.show(&hud_camera_query, &mut tooltip_ui),
    }
}
