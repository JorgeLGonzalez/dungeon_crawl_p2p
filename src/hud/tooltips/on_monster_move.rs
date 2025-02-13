use super::{TooltipToggleTrigger, TooltipUIQuery};
use crate::{monsters::MonsterMovesEvent, prelude::*};

/// Hides the tooltip when a monster with it moves.
pub fn on_monster_move(
    mut commands: Commands,
    mut move_events: EventReader<MonsterMovesEvent>,
    tooltip_ui: TooltipUIQuery,
) {
    let (.., tooltip) = tooltip_ui.single();
    let Some(entity_with_tooltip) = tooltip.entity else {
        return;
    };

    if move_events.read().any(|e| entity_with_tooltip == e.monster) {
        commands.trigger(TooltipToggleTrigger::Hide);
    }
}
