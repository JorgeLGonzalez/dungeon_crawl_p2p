use super::*;
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

/// Show or hide tooltip based on mouse movement
pub fn on_mouse_move(
    mut commands: Commands,
    mut cursor_events: EventReader<CursorMoved>,
    camera_query: CameraQuery,
    local_players: Res<LocalPlayers>,
    players: PlayerQuery,
    tooltip_entities: TooltipEntityQuery,
    tooltip_ui: TooltipUIQuery,
    windows: WindowQuery,
) {
    if cursor_events.is_empty() {
        return;
    }
    cursor_events.clear();

    let toggle = MouseBasedTooltipToggleFactory::new(
        &camera_query,
        &tooltip_ui,
        &tooltip_entities,
        &windows,
    )
    .with_player_fov(&local_players, &players)
    .create(&tooltip_entities);

    if let Some(toggle) = toggle {
        commands.trigger(toggle);
    }
}
