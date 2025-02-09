use super::*;
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

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

    if let Some(toggle) = TooltipDeterminerBuilder::new(&tooltip_ui)
        .mouse_info(&camera_query, &windows)
        .add_player_info(&local_players, &players)
        .build()
        .determine_from_mouse_move(&tooltip_entities)
    {
        commands.trigger(toggle);
    }
}
