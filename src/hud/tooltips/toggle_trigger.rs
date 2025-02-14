use super::*;
use crate::player::LocalPlayer;
use bevy::{math::VectorSpace, prelude::*};
use bevy_ggrs::LocalPlayers;

/// The event triggered from tooltip systems monitoring mouse, player and monster
/// movement. Observed by [`toggle_tooltip`].
#[derive(Event, Debug)]
pub enum TooltipToggleTrigger {
    /// Hide the active tooltip
    Hide,
    /// Show a tooltip on the entity under the mouse cursor
    ShowOnMouseCursor(TooltipDisplayInfo<MouseTooltip>),
    /// Show a tooltip for the entity on which the player is standing
    ShowOnPlayer(TooltipDisplayInfo<PlayerTooltip>),
}

/// Builds the proper TooltipToggleTrigger variant based on a mouse movement
/// event.
pub struct TooltipToggleTriggerBuilder {
    /// whether mouse is in local player's FOV
    in_fov: bool,
    mouse_pos: Option<MousePosition>,
    tooltip: Option<TooltipInfo>,
}

impl TooltipToggleTriggerBuilder {
    pub fn new(camera_query: &CameraQuery, windows: &WindowQuery) -> Self {
        Self {
            in_fov: false,
            mouse_pos: MousePosition::try_new(camera_query, windows),
            tooltip: None,
        }
    }

    pub fn build(self, tooltip_entities: &TooltipEntityQuery) -> Option<TooltipToggleTrigger> {
        let tooltip = self.tooltip.expect("Tooltip not set");

        let Some(mouse_pos) = self.mouse_pos else {
            info!("no mouse pos");
            return tooltip.active().then_some(TooltipToggleTrigger::Hide);
        };

        if !self.in_fov {
            info!("mouse not in FOV");
            return tooltip.active().then_some(TooltipToggleTrigger::Hide);
        }
        // info!("mouse in FOV");

        // mouse moved but not off of active entity
        if tooltip.active() && tooltip.hit_test(mouse_pos.game) {
            info!("mouse still over tooltip entity");
            return None;
        }

        tooltip_entities
            .iter()
            .find(|(.., transform)| hit_test(mouse_pos.game, transform))
            .map(|(entity, label, _)| {
                TooltipDisplayInfo::new(MouseTooltip(mouse_pos.screen), entity, label.0.clone())
            })
            .map(TooltipToggleTrigger::ShowOnMouseCursor)
            .or_else(|| tooltip.active().then_some(TooltipToggleTrigger::Hide))
    }

    pub fn with_player_fov(mut self, local_players: &LocalPlayers, players: &PlayerQuery) -> Self {
        self.in_fov = self
            .mouse_pos
            .as_ref()
            .map_or(false, |mp| mp.in_player_fov(local_players, players));

        self
    }

    pub fn with_tooltip(
        mut self,
        tooltip_ui: &TooltipUIQuery,
        tooltip_entities: &TooltipEntityQuery,
    ) -> Self {
        self.tooltip = Some(TooltipInfo::new(tooltip_ui, tooltip_entities));

        self
    }
}

struct MousePosition {
    pub game: Vec2,
    pub screen: Vec2,
}

impl MousePosition {
    pub fn try_new(camera_query: &CameraQuery, windows: &WindowQuery) -> Option<Self> {
        let Some(screen) = windows.single().cursor_position() else {
            return None;
        };

        let (camera, camera_transform) = camera_query.single();

        if let Ok(game) = camera.viewport_to_world_2d(camera_transform, screen) {
            Some(Self { game, screen })
        } else {
            None
        }
    }

    pub fn in_player_fov(&self, local_players: &LocalPlayers, players: &PlayerQuery) -> bool {
        players
            .iter()
            .find(|(player, ..)| LocalPlayer::is_local(player, local_players))
            // .map(|(_, fov, ..)| fov.visible_tiles.contains_key(&self.game.as_ivec2()))
            .map(|(_, fov, ..)| {
                info!(
                    "pos: {} {} {} {} {}",
                    self.game,
                    self.game.as_ivec2(),
                    self.game.round().as_ivec2(),
                    fov.visible_tiles
                        .keys()
                        .next()
                        .unwrap_or(&Vec2::ZERO.as_ivec2()),
                    fov.visible_tiles
                        .keys()
                        .last()
                        .unwrap_or(&Vec2::ZERO.as_ivec2())
                );
                fov.visible_tiles
                    .contains_key(&self.game.round().as_ivec2())
            })
            .expect("No local player!")
    }
}

struct TooltipInfo {
    active: bool,
    transform: Option<Transform>,
}

impl TooltipInfo {
    pub fn new(tooltip_ui: &TooltipUIQuery, tooltip_entities: &TooltipEntityQuery) -> Self {
        let (.., tooltip) = tooltip_ui.single();

        if let Some(entity) = tooltip.entity {
            let transform = tooltip_entities
                .get(entity)
                .map(|(.., transform)| transform.clone())
                .ok();

            Self {
                active: true,
                transform,
            }
        } else {
            Self {
                active: false,
                transform: None,
            }
        }
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn hit_test(&self, pos: Vec2) -> bool {
        self.transform.is_some_and(|t| hit_test(pos, &t))
    }
}

fn hit_test(mouse_pos: Vec2, target_transform: &Transform) -> bool {
    let tile_pos = target_transform.translation.truncate();
    let min = tile_pos - 0.5;
    let max = tile_pos + 0.5;

    info!("tile: {min} {max} mouse: {mouse_pos}");

    mouse_pos.x > min.x && mouse_pos.x < max.x && mouse_pos.y > min.y && mouse_pos.y < max.y
}
