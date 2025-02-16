use super::*;
use crate::player::{LocalPlayer, PlayerMovesEvent};
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

/// Show or hide tooltip based player movement
pub fn on_player_move(
    mut commands: Commands,
    mut player_movement_events: EventReader<PlayerMovesEvent>,
    local_players: Res<LocalPlayers>,
    tooltip_entities: TooltipEntityQuery,
    tooltip_ui: TooltipUIQuery,
) {
    let Some(event) = player_movement_events.read().next() else {
        return;
    };

    let tooltipped_entity = TooltipInfo::entity(&tooltip_ui);

    if LocalPlayer::is_local_player_id(event.player_id, &local_players) {
        if let Some(toggle) = create_toggle(&event, &tooltip_entities, tooltipped_entity) {
            commands.trigger(toggle);
        }
    } else if tooltipped_entity.filter(|e| *e == event.player).is_some() {
        // Hide active tooltip on the remote player if they moved
        commands.trigger(TooltipToggleTrigger::Hide);
    }
}

/// Check all entities that can have a tooltip and create the proper toggle based
/// on local player's position
fn create_toggle(
    event: &PlayerMovesEvent,
    tooltip_entities: &TooltipEntityQuery,
    entity_with_tooltip: Option<Entity>,
) -> Option<TooltipToggleTrigger> {
    tooltip_entities
        .iter()
        .find_map(|q| create_tooltip_if_on_entity(q, event.player, event.pos))
        .map(TooltipToggleTrigger::Show)
        .or_else(|| entity_with_tooltip.and(Some(TooltipToggleTrigger::Hide)))
}

/// Create a TooltipDisplayInfo for a PlayerTooltip if the player is over the given
/// entity.
fn create_tooltip_if_on_entity(
    (entity, label, transform): (Entity, &TooltipLabel, &Transform),
    player: Entity,
    player_pos: IVec2,
) -> Option<TooltipDisplayInfo> {
    if entity == player {
        // Ignore the player entity itself, since the player is always at their own pos!
        return None;
    }

    let entity_pos = transform.translation.truncate();

    (entity_pos.as_ivec2() == player_pos).then_some(TooltipDisplayInfo::new(
        entity_pos,
        entity,
        label.0.clone(),
    ))
}
