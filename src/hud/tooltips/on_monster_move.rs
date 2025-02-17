use super::{TooltipInfo, TooltipToggleTrigger, TooltipUIQuery};
use crate::{monsters::MonsterMovesEvent, prelude::*};

/// Hides the tooltip on a monster that has moved.
pub fn on_monster_move(
    mut commands: Commands,
    mut move_events: EventReader<MonsterMovesEvent>,
    tooltip_ui: TooltipUIQuery,
) {
    let Some(tooltipped_entity) = TooltipInfo::entity(&tooltip_ui) else {
        return;
    };

    if move_events.read().any(|e| tooltipped_entity == e.monster) {
        commands.trigger(TooltipToggleTrigger::Hide);
    }
}
