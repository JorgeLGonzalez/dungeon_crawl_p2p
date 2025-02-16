use super::*;
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

/// Builds the proper TooltipToggleTrigger variant based on a mouse movement
/// event.
pub struct TooltipToggleTriggerBuilder {
    /// whether mouse is in local player's FOV
    in_fov: bool,
    mouse_pos: Option<MousePosition>,
    tooltip: TooltipInfo,
}

impl TooltipToggleTriggerBuilder {
    pub fn new(
        camera_query: &CameraQuery,
        tooltip_ui: &TooltipUIQuery,
        tooltip_entities: &TooltipEntityQuery,
        windows: &WindowQuery,
    ) -> Self {
        Self {
            in_fov: false,
            mouse_pos: MousePosition::try_new(camera_query, windows),
            tooltip: TooltipInfo::new(tooltip_ui, tooltip_entities),
        }
    }

    /// Build the TooltipToggleTrigger variant (or None, if no toggle is needed).
    /// Perform simple checks first, then check all entities that can have a tooltip.
    pub fn build(self, tooltip_entities: &TooltipEntityQuery) -> Option<TooltipToggleTrigger> {
        if !self.in_fov {
            return self.tooltip.active().then_some(TooltipToggleTrigger::Hide);
        }

        let Some(mouse_pos) = self.mouse_pos else {
            // mouse is presumably off screen
            return self.tooltip.active().then_some(TooltipToggleTrigger::Hide);
        };

        if self.tooltip.active() && self.tooltip.hit_test(mouse_pos.game) {
            // mouse moved but not off of active tooltipped entity
            return None;
        }

        self.create_toggle(tooltip_entities)
    }

    pub fn with_player_fov(mut self, local_players: &LocalPlayers, players: &PlayerQuery) -> Self {
        self.in_fov = self
            .mouse_pos
            .as_ref()
            .map_or(false, |mp| mp.in_player_fov(local_players, players));

        self
    }

    /// Check all entities that can have a tooltip and create the proper toggle
    /// trigger if applicable
    fn create_toggle(&self, tooltip_entities: &TooltipEntityQuery) -> Option<TooltipToggleTrigger> {
        let mouse_pos = self.mouse_pos.expect("Mouse position should be set");

        tooltip_entities
            .iter()
            .find_map(|q| create_tooltip_if_on_entity(q, mouse_pos))
            .map(TooltipToggleTrigger::Show)
            .or_else(|| self.tooltip.active().then_some(TooltipToggleTrigger::Hide))
    }
}

/// Create a TooltipDisplayInfo for a MouseTooltip if the mouse is over the given
/// entity.
fn create_tooltip_if_on_entity(
    (entity, label, transform): (Entity, &TooltipLabel, &Transform),
    mouse_pos: MousePosition,
) -> Option<TooltipDisplayInfo> {
    let entity_pos = transform.translation.truncate();

    (entity_pos.as_ivec2() == mouse_pos.game).then_some(TooltipDisplayInfo::new(
        entity_pos,
        entity,
        label.0.clone(),
    ))
}
