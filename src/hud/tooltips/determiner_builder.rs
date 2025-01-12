use super::{
    determiner::TooltipDeterminer,
    queries::{CameraQuery, PlayerQuery, TooltipUIQuery, WindowQuery},
};
use crate::player::LocalPlayer;
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

/// Builds a [`TooltipDeterminer`] from the current state of the game.
#[derive(Default)]
pub struct TooltipDeterminerBuilder {
    game_pos: Option<Vec2>,
    in_fov: bool,
    mouse_moved: bool,
    mouse_pos: Option<Vec2>,
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
        let game_pos = mouse_pos.map(|pos| {
            camera
                .viewport_to_world_2d(camera_transform, pos)
                .expect("Inconceivable!")
        });

        Self {
            mouse_moved,
            mouse_pos,
            game_pos,
            ..default()
        }
    }

    /// Builds the [`TooltipDeterminer`] from the current state of the game.
    pub fn build(self) -> TooltipDeterminer {
        TooltipDeterminer::new(
            self.game_pos,
            self.in_fov,
            self.mouse_moved,
            self.mouse_pos,
            self.tooltipped_entity,
        )
    }

    /// Determines whether the mouse cursor is over a game position within the
    /// local player's field of view.
    pub fn local_player_fov(self, local_players: &LocalPlayers, players: &PlayerQuery) -> Self {
        let in_fov = self.game_pos.is_some_and(|pos| {
            players
                .iter()
                .find(|(player, _)| LocalPlayer::is_local(player, &local_players))
                .map(|(_, fov)| fov)
                .expect("No local player to follow!")
                .visible_tiles
                .contains_key(&pos.as_ivec2())
        });

        Self { in_fov, ..self }
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
