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
    // TODO do nothing if mouse did not move
    if let Some(toggle_action) = TooltipDeterminerBuilder::new(&tooltip_ui)
        .check_mouse_movement(&mut cursor_events)
        .and_then(|b| {
            b.mouse_info(&camera_query, &windows)
                .add_player_info(&local_players, &players)
                .build()
                .determine(&tooltip_entities)
        })
    {
        commands.trigger(toggle_action);
    }
}
