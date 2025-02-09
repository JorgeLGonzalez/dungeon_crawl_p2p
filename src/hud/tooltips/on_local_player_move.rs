use super::*;
use crate::player::{LocalPlayer, PlayerMovesEvent};
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

pub fn on_local_player_move(
    mut commands: Commands,
    mut player_movement_events: EventReader<PlayerMovesEvent>,
    local_players: Res<LocalPlayers>,
    tooltip_entities: TooltipEntityQuery,
) {
    let Some(event) = player_movement_events.read().next() else {
        return;
    };

    if !LocalPlayer::is_local_player_id(event.player_id, &local_players) {
        return;
    }

    if let Some(toggle_action) = toggle_trigger(&event, &tooltip_entities) {
        commands.trigger(toggle_action);
    }
}

fn toggle_trigger(
    event: &PlayerMovesEvent,
    tooltip_entities: &TooltipEntityQuery,
) -> Option<TooltipToggleTrigger> {
    let player_pos = event.pos.as_vec2();

    tooltip_entities
        .iter()
        .filter(|(entity, ..)| *entity != event.player)
        .find(|(.., transform)| transform.translation.truncate() == player_pos)
        .map(|(entity, label, _)| TooltipDisplayInfo::new(PlayerTooltip, entity, label.0.clone()))
        .map(TooltipToggleTrigger::ShowOnPlayer)
        .or(Some(TooltipToggleTrigger::Hide))
}
