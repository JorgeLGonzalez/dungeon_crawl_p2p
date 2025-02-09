use super::*;
use crate::player::PlayerMovesEvent;
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

pub fn on_local_player_move(
    mut commands: Commands,
    mut player_movement_events: EventReader<PlayerMovesEvent>,
    local_players: Res<LocalPlayers>,
    tooltip_entities: TooltipEntityQuery,
    tooltip_ui: TooltipUIQuery,
) {
    // TODO do nothing if player did not move
    if let Some(toggle_action) = TooltipDeterminerBuilder::new(&tooltip_ui)
        .check_local_player_movement(&local_players, &mut player_movement_events)
        .and_then(|b| b.build().determine(&tooltip_entities))
    {
        commands.trigger(toggle_action);
    }
}
