use super::*;
use bevy::{prelude::*, utils::hashbrown::HashSet};

pub struct TooltipToggleFactory {
    ignore_set: HashSet<Entity>,
    is_tooltip_active: bool,
    pos: IVec2,
}

impl TooltipToggleFactory {
    pub fn new(pos: IVec2, is_tooltip_active: bool) -> Self {
        Self {
            pos,
            ignore_set: HashSet::new(),
            is_tooltip_active,
        }
    }

    /// Check  all entities that can have a tooltip and create the proper toggle
    /// trigger
    pub fn create(&self, tooltip_entities: &TooltipEntityQuery) -> Option<TooltipToggleTrigger> {
        tooltip_entities
            .iter()
            .filter(|(entity, ..)| !self.ignore_set.contains(entity))
            .find_map(|q| self.create_tooltip_if_on_entity(q))
            .map(TooltipToggleTrigger::Show)
            .or_else(|| self.is_tooltip_active.then_some(TooltipToggleTrigger::Hide))
    }

    /// Ignore the given entity when checking for those that support tooltips
    /// Used to skip checking the player entity when player movement might trigger
    /// tooltip activation.
    pub fn ignore(mut self, entity: Entity) -> Self {
        self.ignore_set.insert(entity);

        self
    }

    fn create_tooltip_if_on_entity(
        &self,
        (entity, label, transform): (Entity, &TooltipLabel, &Transform),
    ) -> Option<TooltipDisplayInfo> {
        let entity_pos = transform.translation.truncate();

        (entity_pos.as_ivec2() == self.pos).then_some(TooltipDisplayInfo::new(
            entity_pos,
            entity,
            label.0.clone(),
        ))
    }
}
