use super::{TooltipEntityQuery, TooltipUIQuery};
use bevy::prelude::*;

pub struct TooltipInfo {
    /// Whether a tooltip is being displayed
    active: bool,
    /// The transform of the entity over which the tooltip is displayed.
    /// Note you may have an active tooltip w/o an entity transform for example
    /// when the entity was destroyed and we haven't yet hidden the tooltip.
    transform: Option<Transform>,
}

impl TooltipInfo {
    pub fn new(tooltip_ui: &TooltipUIQuery, tooltip_entities: &TooltipEntityQuery) -> Self {
        let Some(entity) = Self::entity(tooltip_ui) else {
            return Self {
                active: false,
                transform: None,
            };
        };

        let transform = tooltip_entities
            .get(entity)
            .map(|(.., transform)| transform.clone())
            .ok();

        Self {
            active: true,
            transform,
        }
    }

    /// Entity with active tooltip
    pub fn entity(tooltip_ui: &TooltipUIQuery) -> Option<Entity> {
        let (.., tooltip) = tooltip_ui.single();

        tooltip.entity
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn hit_test(&self, pos: IVec2) -> bool {
        self.transform
            .is_some_and(|t| pos == t.translation.truncate().as_ivec2())
    }
}
