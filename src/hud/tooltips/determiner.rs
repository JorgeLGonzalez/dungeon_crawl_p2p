use super::*;
use bevy::prelude::*;

#[derive(Debug, Default, Eq, PartialEq)]
pub enum Mover {
    /// The mouse cursor moved
    Mouse,
    /// A monster may have moved
    #[default]
    Unknown,
}

/// Determines whether to show or hide a tooltip based on the current state of
/// the game.
pub struct TooltipDeterminer {
    game_pos: Option<Vec2>,
    in_fov: bool,
    mover: Mover,
    mouse_pos: Option<Vec2>,
    player_pos: Vec2,
    tooltipped_entity: Option<Entity>,
}

impl TooltipDeterminer {
    pub fn new(
        game_pos: Option<Vec2>,
        in_fov: bool,
        mover: Mover,
        mouse_pos: Option<Vec2>,
        player_pos: Vec2,
        tooltipped_entity: Option<Entity>,
    ) -> Self {
        Self {
            game_pos,
            in_fov,
            mover,
            mouse_pos,
            player_pos,
            tooltipped_entity,
        }
    }

    /// Determine the tooltip toggle action based on the game state.
    pub fn determine(
        &mut self,
        tooltip_entities: &TooltipEntityQuery,
    ) -> Option<TooltipToggleTrigger> {
        if let Some(info) = self.mouse_movement(tooltip_entities) {
            Some(TooltipToggleTrigger::ShowOnMouseCursor(info))
        } else if self.active_tooltip() && !self.still_on_entity(tooltip_entities) {
            Some(TooltipToggleTrigger::Hide)
        } else {
            None
        }
    }

    fn active_tooltip(&self) -> bool {
        self.tooltipped_entity.is_some()
    }

    /// Check whether the mouse cursor (converted to `game_pos`) is within the
    /// bounds of the entity's tile Tiles are 1x1 in size.
    fn hit_test(&self, transform: &Transform) -> bool {
        let Some(point) = self.game_pos else {
            return false;
        };

        let tile_pos = transform.translation.truncate();
        let min = tile_pos - 0.5;
        let max = tile_pos + 0.5;

        point.x > min.x && point.x < max.x && point.y > min.y && point.y < max.y
    }

    /// Check if a mouse cursor movement should trigger a tooltip display.
    fn mouse_movement(
        &self,
        tooltip_entities: &TooltipEntityQuery,
    ) -> Option<TooltipDisplayInfo<MouseTooltip>> {
        if self.mover != Mover::Mouse || !self.in_fov || self.still_on_entity(tooltip_entities) {
            // Bail out early based on cheap tests. Obviously no need to show if:
            // - or mouse has not moved, so it has not moved ONTO anything
            // - mouse not in FOV
            // - or mouse is still on the entity with the active tooltip
            return None;
        }

        let Some((entity, label)) = tooltip_entities
            .iter()
            .find(|(.., transform)| self.hit_test(transform))
            .map(|(entity, label, _)| (entity, label.0.clone()))
        else {
            return None;
        };

        Some(TooltipDisplayInfo::new(
            MouseTooltip(self.mouse_pos.unwrap()),
            entity,
            label,
        ))
    }

    fn still_on_entity(&self, tooltip_entities: &TooltipEntityQuery) -> bool {
        let Some(entity) = self.tooltipped_entity else {
            return false;
        };
        let Ok((.., transform)) = tooltip_entities.get(entity) else {
            warn!("Tooltip entity not found: {entity:?}. Killed off?");
            return false;
        };

        match self.mover {
            Mover::Mouse => self.hit_test(transform),
            Mover::Unknown => {
                (self.player_pos == transform.translation.truncate()) || self.hit_test(transform)
            }
        }
    }
}
