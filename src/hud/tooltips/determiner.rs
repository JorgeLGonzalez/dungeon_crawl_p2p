use super::{hider::TooltipHider, queries::TooltipEntityQuery, shower::TooltipShower};
use bevy::prelude::*;

pub enum TooltipToggleAction {
    Hide(TooltipHider),
    None,
    Show(TooltipShower),
}

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

    fn find_entity_to_tooltip(
        &self,
        tooltip_entities: &TooltipEntityQuery,
    ) -> Option<(Entity, String)> {
        tooltip_entities
            .iter()
            .find(|(.., transform)| self.hit_test(transform))
            .map(|(entity, label, _)| (entity, label.0.clone()))
    }

    fn hit_test(&self, transform: &Transform) -> bool {
        let Some(point) = self.game_pos else {
            return false;
        };

        let tile_pos = transform.translation.truncate();
        let min = tile_pos - 0.5;
        let max = tile_pos + 0.5;

        point.x > min.x && point.x < max.x && point.y > min.y && point.y < max.y
    }

    fn try_create_shower(&self, tooltip_entities: &TooltipEntityQuery) -> Option<TooltipShower> {
        if !self.in_fov || !self.mouse_moved || self.still_on_entity(tooltip_entities) {
            // Bail out early based on cheap tests. Obviously no need to show if:
            // - mouse not in FOV
            // - or mouse has not moved, so it has not moved ONTO anything
            // - or mouse is still on the entity with the active tooltip
            return None;
        }

        if let Some((tooltip_entity, tooltip_label)) = self.find_entity_to_tooltip(tooltip_entities)
        {
            Some(TooltipShower::new(
                self.mouse_pos.unwrap(),
                tooltip_entity,
                tooltip_label,
            ))
        } else {
            None
        }
    }

    fn still_on_entity(&self, tooltip_entities: &TooltipEntityQuery) -> bool {
        if let Some(entity) = self.tooltipped_entity {
            let (.., transform) = tooltip_entities.get(entity).expect("Inconceivable!");

            self.hit_test(transform)
        } else {
            false
        }
    }
}
