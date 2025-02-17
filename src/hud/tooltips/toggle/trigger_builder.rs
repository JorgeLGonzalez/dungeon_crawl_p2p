use super::*;
use crate::{player::LocalPlayer, prelude::*};
use bevy_ggrs::LocalPlayers;

/// Builds the proper TooltipToggleTrigger variant based on a mouse movement
/// event.
pub struct MouseBasedTooltipToggleFactory {
    /// whether mouse is in local player's FOV
    in_fov: bool,
    is_tooltip_visible: bool,
    mouse_pos: Option<IVec2>,
    tooltip_transform: Option<Transform>,
}

impl MouseBasedTooltipToggleFactory {
    pub fn new(
        camera_query: &CameraQuery,
        tooltip_ui: &TooltipUIQuery,
        tooltip_entities: &TooltipEntityQuery,
        windows: &WindowQuery,
    ) -> Self {
        let tooltipped_entity = TooltipInfo::entity(tooltip_ui);
        let tooltip_transform = tooltipped_entity.and_then(|entity| {
            tooltip_entities
                .get(entity)
                .map(|(.., transform)| transform.clone())
                .ok()
        });

        Self {
            in_fov: false,
            is_tooltip_visible: tooltipped_entity.is_some(),
            mouse_pos: Self::create_mouse_pos(camera_query, windows),
            tooltip_transform,
        }
    }

    /// Build the TooltipToggleTrigger variant (or None, if no toggle is needed).
    /// Perform simple checks first, then check all entities that can have a tooltip.
    pub fn create(self, tooltip_entities: &TooltipEntityQuery) -> Option<TooltipToggleTrigger> {
        if !self.in_fov {
            return self.maybe_hide();
        }

        let Some(mouse_pos) = self.mouse_pos else {
            // mouse is presumably off screen
            return self.maybe_hide();
        };

        if self.still_on_tooltip(mouse_pos) {
            return None;
        }

        TooltipToggleFactory::new(mouse_pos, self.is_tooltip_visible).create(tooltip_entities)
    }

    pub fn with_player_fov(mut self, local_players: &LocalPlayers, players: &PlayerQuery) -> Self {
        let Some(mouse_pos) = self.mouse_pos else {
            return self;
        };

        self.in_fov = players
            .iter()
            .find(|(player, ..)| LocalPlayer::is_local(player, local_players))
            .map(|(_, fov, ..)| fov.visible_tiles.contains_key(&mouse_pos))
            .expect("No local player!");

        self
    }

    fn create_mouse_pos(camera_query: &CameraQuery, windows: &WindowQuery) -> Option<IVec2> {
        let Some(screen_pos) = windows.single().cursor_position() else {
            return None;
        };

        let (camera, camera_transform) = camera_query.single();

        camera
            .viewport_to_world_2d(camera_transform, screen_pos)
            .map(|game| game.round().as_ivec2())
            .ok()
    }

    /// Hide the tooltip if it is visible
    fn maybe_hide(&self) -> Option<TooltipToggleTrigger> {
        self.is_tooltip_visible
            .then_some(TooltipToggleTrigger::Hide)
    }

    /// Is the tooltip visible and the mouse is still on the same position?
    /// This means the mouse has moved, but not off of the entity with the
    /// active tooltip
    fn still_on_tooltip(&self, mouse_pos: IVec2) -> bool {
        self.tooltip_transform
            .is_some_and(|t| mouse_pos == t.translation.truncate().as_ivec2())
    }
}
