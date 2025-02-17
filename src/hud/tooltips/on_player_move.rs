use super::*;
use crate::{
    player::{LocalPlayer, PlayerMovesEvent},
    prelude::*,
};
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
        let toggle = TooltipToggleFactory::new(event.pos, tooltipped_entity.is_some())
            .ignore(event.player)
            .create(&tooltip_entities);

        if let Some(toggle) = toggle {
            commands.trigger(toggle);
        }
    } else if tooltipped_entity.filter(|e| *e == event.player).is_some() {
        // Hide active tooltip on the remote player if they moved
        commands.trigger(TooltipToggleTrigger::Hide);
    }
}
