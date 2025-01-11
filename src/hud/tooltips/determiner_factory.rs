use super::{
    determiner::TooltipDeterminer,
    queries::{CameraQuery, PlayerQuery, TooltipUIQuery, WindowQuery},
};
use crate::player::LocalPlayer;
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

pub struct TooltipDeterminerFactory;

impl TooltipDeterminerFactory {
    pub fn create(
        camera_query: &CameraQuery,
        cursor_events: &mut EventReader<CursorMoved>,
        local_players: &LocalPlayers,
        players: &PlayerQuery,
        tooltip_ui: &TooltipUIQuery,
        windows: &WindowQuery,
    ) -> TooltipDeterminer {
        let (.., tooltip) = tooltip_ui.single();
        let tooltipped_entity = tooltip.entity;

        let mouse_moved = !cursor_events.is_empty();
        cursor_events.clear();

        let mouse_pos = windows.single().cursor_position();
        let game_pos = Self::to_game_pos(mouse_pos, camera_query);
        let in_fov = game_pos.is_some_and(|pos| Self::in_player_fov(pos, local_players, players));

        TooltipDeterminer::new(game_pos, in_fov, mouse_moved, mouse_pos, tooltipped_entity)
    }

    fn in_player_fov(game_pos: Vec2, local_players: &LocalPlayers, players: &PlayerQuery) -> bool {
        players
            .iter()
            .find(|(player, _)| LocalPlayer::is_local(player, &local_players))
            .map(|(_, fov)| fov)
            .expect("No local player to follow!")
            .visible_tiles
            .contains_key(&game_pos.as_ivec2())
    }

    fn to_game_pos(mouse_pos: Option<Vec2>, camera_query: &CameraQuery) -> Option<Vec2> {
        let (camera, camera_transform) = camera_query.single();

        mouse_pos.map(|pos| {
            camera
                .viewport_to_world_2d(camera_transform, pos)
                .expect("Inconceivable!")
        })
    }
}
