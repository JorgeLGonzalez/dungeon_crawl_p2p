use super::{hider::TooltipHider, shower::TooltipShower};
use crate::hud::TooltipLabel;
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

    pub fn determine(
        &self,
        finder: impl FnOnce(&Self) -> Option<(Entity, String)>,
        transform_getter: &impl Fn(Entity) -> Transform,
    ) -> TooltipToggleAction {
        if let Some(shower) = self.try_create_shower(finder, transform_getter) {
            TooltipToggleAction::Show(shower)
        } else if self.active_tooltip() && !self.still_on_entity(transform_getter) {
            TooltipToggleAction::Hide(TooltipHider)
        } else {
            TooltipToggleAction::None
        }
    }

    pub fn test_entity(
        &self,
        entity: Entity,
        label: &TooltipLabel,
        transform: &Transform,
    ) -> Option<(Entity, String)> {
        if self.hit_test(transform) {
            Some((entity, label.0.clone()))
        } else {
            None
        }
    }

    fn active_tooltip(&self) -> bool {
        self.tooltipped_entity.is_some()
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

    fn still_on_entity(&self, get_transform: &impl Fn(Entity) -> Transform) -> bool {
        if let Some(entity) = self.tooltipped_entity {
            let transform = get_transform(entity);
            self.hit_test(&transform)
        } else {
            false
        }
    }

    fn try_create_shower(
        &self,
        find_entity_to_tooltip: impl FnOnce(&Self) -> Option<(Entity, String)>,
        get_tooltipped_entity_transform: &impl Fn(Entity) -> Transform,
    ) -> Option<TooltipShower> {
        if !self.in_fov
            || !self.mouse_moved
            || self.still_on_entity(get_tooltipped_entity_transform)
        {
            // Bail out early based on cheap tests. Obviously no need to show if:
            // - mouse not in FOV
            // - or mouse has not moved, so it has not moved ONTO anything
            // - or mouse is still on the entity with the active tooltip
            return None;
        }

        if let Some((tooltip_entity, tooltip_label)) = find_entity_to_tooltip(self) {
            Some(TooltipShower::new(
                self.mouse_pos.unwrap(),
                tooltip_entity,
                tooltip_label,
            ))
        } else {
            None
        }
    }
}
