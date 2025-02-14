use super::*;
use crate::player::{LocalPlayer, PlayerMovesEvent};
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

/// Show or hide tooltip based (local or remote) player movement
pub fn on_player_move(
    mut commands: Commands,
    mut player_movement_events: EventReader<PlayerMovesEvent>,
    local_players: Res<LocalPlayers>,
    tooltip_entities: TooltipEntityQuery,
    tooltip_ui: TooltipUIQuery,
) {
    let Some(event) = player_movement_events.read().next() else {
        info!("on_player_move: ignored");
        return;
    };

    let (.., tooltip) = tooltip_ui.single();

    if LocalPlayer::is_local_player_id(event.player_id, &local_players) {
        if let Some(toggle) = toggle_trigger(&event, &tooltip_entities, tooltip.entity) {
            info!("on_player_move: local player {toggle:?}");
            commands.trigger(toggle);
        }
    } else {
        if tooltip.entity.filter(|e| *e == event.player).is_some() {
            // Hide any tooltip on the remote player if they moved
            info!("on_player_move: remote player hide");
            commands.trigger(TooltipToggleTrigger::Hide);
        }
    }
}

fn toggle_trigger(
    event: &PlayerMovesEvent,
    tooltip_entities: &TooltipEntityQuery,
    entity_with_tooltip: Option<Entity>,
) -> Option<TooltipToggleTrigger> {
    let player_pos = event.pos.as_vec2();

    tooltip_entities
        .iter()
        .filter(|(entity, ..)| *entity != event.player)
        .find(|(.., transform)| transform.translation.truncate() == player_pos)
        .map(|(entity, label, _)| TooltipDisplayInfo::new(PlayerTooltip, entity, label.0.clone()))
        .map(TooltipToggleTrigger::ShowOnPlayer)
        .or_else(|| entity_with_tooltip.and(Some(TooltipToggleTrigger::Hide)))
}
