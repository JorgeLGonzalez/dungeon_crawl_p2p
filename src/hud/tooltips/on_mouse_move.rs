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

    let toggle = TooltipToggleTriggerBuilder::new(&camera_query, &windows)
        .with_player_fov(&local_players, &players)
        .with_tooltip(&tooltip_ui, &tooltip_entities)
        .build(&tooltip_entities);

    if let Some(toggle) = toggle {
        commands.trigger(toggle);
    }
}
