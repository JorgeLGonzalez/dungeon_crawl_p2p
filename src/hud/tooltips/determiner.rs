use super::*;
use bevy::prelude::*;

/// Determines whether to show or hide a tooltip based on the current state of
/// the game.
pub struct TooltipDeterminer {
    game_pos: Option<Vec2>,
    in_fov: bool,
    mouse_pos: Option<Vec2>,
    player_pos: Vec2,
    tooltipped_entity: Option<Entity>,
}

impl TooltipDeterminer {
    pub fn new(
        game_pos: Option<Vec2>,
        in_fov: bool,
        mouse_pos: Option<Vec2>,
        player_pos: Vec2,
        tooltipped_entity: Option<Entity>,
    ) -> Self {
        Self {
            game_pos,
            in_fov,
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
        if self.active_tooltip() && !self.still_on_entity(tooltip_entities) {
            Some(TooltipToggleTrigger::Hide)
        } else {
            None
        }
    }

    pub fn determine_from_mouse_move(
        &self,
        tooltip_entities: &TooltipEntityQuery,
    ) -> Option<TooltipToggleTrigger> {
        if !self.in_fov && self.active_tooltip() {
            return Some(TooltipToggleTrigger::Hide);
        }
        let Some(mouse_pos) = self.mouse_pos else {
            if self.active_tooltip() {
                return Some(TooltipToggleTrigger::Hide);
            } else {
                return None;
            }
        };

        if self.still_on_entity(tooltip_entities) {
            return None;
        }

        tooltip_entities
            .iter()
            .find(|(.., transform)| self.hit_test(transform))
            .map(|(entity, label, _)| {
                TooltipDisplayInfo::new(MouseTooltip(mouse_pos), entity, label.0.clone())
            })
            .map(TooltipToggleTrigger::ShowOnMouseCursor)
            .or_else(|| {
                if self.active_tooltip() {
                    Some(TooltipToggleTrigger::Hide)
                } else {
                    None
                }
            })
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

    fn still_on_entity(&self, tooltip_entities: &TooltipEntityQuery) -> bool {
        let Some(entity) = self.tooltipped_entity else {
            return false;
        };
        let Ok((.., transform)) = tooltip_entities.get(entity) else {
            warn!("Tooltip entity not found: {entity:?}. Killed off?");
            return false;
        };

        self.player_pos == transform.translation.truncate() || self.hit_test(transform)
    }
}
