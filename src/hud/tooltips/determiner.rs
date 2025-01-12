use super::{hider::TooltipHider, queries::TooltipEntityQuery, shower::TooltipShower};
use bevy::prelude::*;

/// The action to take to show or hide a tooltip (or no action).
pub enum TooltipToggleAction {
    /// Hide the tooltip using the provided hider.
    Hide(TooltipHider),
    /// Do nothing.
    None,
    /// Show the tooltip using the provided shower.
    Show(TooltipShower),
}

/// Determines whether to show or hide a tooltip based on the current state of
/// the game.
pub struct TooltipDeterminer {
    game_pos: Option<Vec2>,
    in_fov: bool,
    mouse_moved: bool,
    mouse_pos: Option<Vec2>,
    tooltipped_entity: Option<Entity>,
}

impl TooltipDeterminer {
    pub fn new(
        game_pos: Option<Vec2>,
        in_fov: bool,
        mouse_moved: bool,
        mouse_pos: Option<Vec2>,
        tooltipped_entity: Option<Entity>,
    ) -> Self {
        Self {
            game_pos,
            in_fov,
            mouse_moved,
            mouse_pos,
            tooltipped_entity,
        }
    }

    /// Determine the tooltip toggle action based on the game state.
    pub fn determine(&mut self, tooltip_entities: &TooltipEntityQuery) -> TooltipToggleAction {
        if let Some(shower) = self.try_create_shower(tooltip_entities) {
            TooltipToggleAction::Show(shower)
        } else if self.active_tooltip() && !self.still_on_entity(tooltip_entities) {
            TooltipToggleAction::Hide(TooltipHider)
        } else {
            TooltipToggleAction::None
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

    fn still_on_entity(&self, tooltip_entities: &TooltipEntityQuery) -> bool {
        let Some(entity) = self.tooltipped_entity else {
            return false;
        };
        let Ok((.., transform)) = tooltip_entities.get(entity) else {
            warn!("Tooltip entity not found: {entity:?}. Killed off?");
            return false;
        };

        self.hit_test(transform)
    }

    fn try_create_shower(&self, tooltip_entities: &TooltipEntityQuery) -> Option<TooltipShower> {
        if !self.in_fov || !self.mouse_moved || self.still_on_entity(tooltip_entities) {
            // Bail out early based on cheap tests. Obviously no need to show if:
            // - mouse not in FOV
            // - or mouse has not moved, so it has not moved ONTO anything
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

        Some(TooltipShower::new(self.mouse_pos.unwrap(), entity, label))
    }
}
