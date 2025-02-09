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
    mut cursor_events: EventReader<CursorMoved>,
    mut player_movement_events: EventReader<PlayerMovesEvent>,
    mut tooltip_ui: TooltipUIQuery,
    camera_query: CameraQuery,
    hud_camera_query: HudCameraQuery,
    local_players: Res<LocalPlayers>,
    players: PlayerQuery,
    tooltip_entities: TooltipEntityQuery,
    windows: WindowQuery,
) {
    let toggle_action = TooltipDeterminerBuilder::new(&camera_query, &mut cursor_events, &windows)
        .local_player_fov(&local_players, &players, &mut player_movement_events)
        .with_tooltip_ui(&mut tooltip_ui)
        .build()
        .determine(&tooltip_entities);

    match toggle_action {
        TooltipToggleAction::Hide(hider) => hider.hide(&mut tooltip_ui),
        TooltipToggleAction::None => {}
        TooltipToggleAction::ShowOnMouseCursor(info) => {
            TooltipShower::new(info).show(&hud_camera_query, &mut tooltip_ui)
        }
        TooltipToggleAction::ShowOnPlayer(info) => TooltipShower::new(info).show(&mut tooltip_ui),
    }
}
