use super::{
    determiner::{Mover, TooltipDeterminer},
    queries::{CameraQuery, PlayerQuery, TooltipUIQuery, WindowQuery},
};
use crate::player::{LocalPlayer, PlayerMovesEvent};
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

/// Builds a [`TooltipDeterminer`] from the current state of the game.
#[derive(Default)]
pub struct TooltipDeterminerBuilder {
    /// Player position, if the player moved. Otherwise mouse cursor position in
    /// game coordinates and if available.
    game_pos: Option<Vec2>,
    /// Whether the mouse cursor is in the local player's field of view. (Always
    /// true if the player moved.)
    in_fov: bool,
    /// Mouse cursor position in screen coordinates, if available.
    mouse_pos: Option<Vec2>,
    /// Whether the tooltip display is toggled based on the local player or mouse
    /// moving, or checked if anything else has changed (e.g. monster moved).
    mover: Mover,
    /// Local player position
    player_pos: Vec2,
    /// Entity currently with tooltip, if any
    tooltipped_entity: Option<Entity>,
}

impl TooltipDeterminerBuilder {
    pub fn new(tooltip_ui: &TooltipUIQuery) -> Self {
        let (.., tooltip) = tooltip_ui.single();

        Self {
            tooltipped_entity: tooltip.entity,
            ..default()
        }
    }

    /// Builds the [`TooltipDeterminer`] from the current state of the game.
    pub fn build(self) -> TooltipDeterminer {
        TooltipDeterminer::new(
            self.game_pos,
            self.in_fov,
            self.mover,
            self.mouse_pos,
            self.player_pos,
            self.tooltipped_entity,
        )
    }

    pub fn check_local_player_movement(
        self,
        local_players: &LocalPlayers,
        player_movement_events: &mut EventReader<PlayerMovesEvent>,
    ) -> Option<Self> {
        let Some(event) = player_movement_events.read().next() else {
            return None;
        };

        if LocalPlayer::is_local_player_id(event.player_id, local_players) {
            let player_pos = event.pos.as_vec2();
            Some(Self {
                game_pos: Some(player_pos),
                in_fov: true,
                mover: Mover::Player(event.player, event.pos),
                mouse_pos: None,
                player_pos,
                tooltipped_entity: None,
            })
        } else {
            None
        }
    }

    /// Set up builder to determine tooltip display reacting to mouse cursor
    /// position and movement (if any).
    /// The mouse position is in screen coordinates and may be unavailable (if
    /// the mouse is not over the window). We calculate the equivalent game
    /// position in game 2D coordinates.
    /// NB: Mouse movement events are cleared.
    pub fn mouse_info(self, camera_query: &CameraQuery, windows: &WindowQuery) -> Self {
        let mouse_pos = windows.single().cursor_position();

        let (camera, camera_transform) = camera_query.single();
        let game_pos =
            mouse_pos.and_then(|pos| camera.viewport_to_world_2d(camera_transform, pos).ok());

        Self {
            game_pos,
            mouse_pos,
            ..self
        }
    }

    pub fn check_mouse_movement(
        self,
        cursor_events: &mut EventReader<CursorMoved>,
    ) -> Option<Self> {
        if cursor_events.is_empty() {
            return None;
        }
        cursor_events.clear();

        Some(Self {
            mover: Mover::Mouse,
            ..self
        })
    }

    /// Add local player position and whether mouse cursor is in their field of
    /// view.
    pub fn add_player_info(self, local_players: &LocalPlayers, players: &PlayerQuery) -> Self {
        let (fov, transform) = players
            .iter()
            .find(|(player, ..)| LocalPlayer::is_local(player, &local_players))
            .map(|(_, fov, transform, ..)| (fov, transform))
            .expect("No local player!");
        let in_fov = self
            .game_pos
            .is_some_and(|pos| fov.visible_tiles.contains_key(&pos.as_ivec2()));

        Self {
            in_fov,
            player_pos: transform.translation.truncate(),
            ..self
        }
    }
}
