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
    game_pos: Option<Vec2>,
    in_fov: bool,
    mouse_moved: bool,
    mouse_pos: Option<Vec2>,
    mover: Mover,
    player_pos: Vec2,
    tooltipped_entity: Option<Entity>,
}

impl TooltipDeterminerBuilder {
    /// Creates the builder, starting off with the mouse cursor position (if
    /// available) in screen coordinates, and the equivalent game position in
    /// game 2D coordinates.
    /// It also checks if the mouse cursor has moved since the last frame.
    /// Importantly, it clears the mouse cursor events.
    pub fn new(
        camera_query: &CameraQuery,
        cursor_events: &mut EventReader<CursorMoved>,
        windows: &WindowQuery,
    ) -> Self {
        let mouse_moved = !cursor_events.is_empty();
        cursor_events.clear();

        let mouse_pos = windows.single().cursor_position();

        let (camera, camera_transform) = camera_query.single();
        let game_pos =
            mouse_pos.and_then(|pos| camera.viewport_to_world_2d(camera_transform, pos).ok());

        let mover = if mouse_moved {
            Mover::Mouse
        } else {
            Mover::Other
        };

        Self {
            game_pos,
            mouse_moved,
            mouse_pos,
            mover,
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

    /// Determines whether the mouse cursor is over a game position within the
    /// local player's field of view.
    pub fn local_player_fov(
        self,
        local_players: &LocalPlayers,
        players: &PlayerQuery,
        player_movement_events: &mut EventReader<PlayerMovesEvent>,
    ) -> Self {
        if let Some(event) = player_movement_events.read().next() {
            if LocalPlayer::is_local_player_id(event.player_id, local_players) {
                return Self {
                    game_pos: Some(event.pos.as_vec2()),
                    in_fov: true,
                    mover: Mover::Player(event.player, event.pos),
                    player_pos: event.pos.as_vec2(),
                    ..self
                };
            }
        }

        let in_fov = self.game_pos.is_some_and(|pos| {
            players
                .iter()
                .find(|(player, ..)| LocalPlayer::is_local(player, &local_players))
                .map(|(_, fov, ..)| fov)
                .expect("No local player to follow!")
                .visible_tiles
                .contains_key(&pos.as_ivec2())
        });

        let player_pos = players
            .iter()
            .find(|(player, ..)| LocalPlayer::is_local(player, &local_players))
            .map(|(_, _, transform, ..)| transform.translation.truncate())
            .expect("No local player to follow!");

        Self {
            in_fov,
            player_pos,
            ..self
        }
    }

    /// Grabs the entity associated with the active tooltip (if applicable).
    pub fn with_tooltip_ui(self, tooltip_ui: &TooltipUIQuery) -> Self {
        let (.., tooltip) = tooltip_ui.single();

        Self {
            tooltipped_entity: tooltip.entity,
            ..self
        }
    }
}
