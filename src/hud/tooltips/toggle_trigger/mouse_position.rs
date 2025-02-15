use super::{CameraQuery, PlayerQuery, WindowQuery};
use crate::{player::LocalPlayer, prelude::*};
use bevy_ggrs::LocalPlayers;

#[derive(Clone, Copy)]
pub(super) struct MousePosition {
    pub game: IVec2,
    pub screen: Vec2,
}

impl MousePosition {
    pub fn try_new(camera_query: &CameraQuery, windows: &WindowQuery) -> Option<Self> {
        let Some(screen) = windows.single().cursor_position() else {
            return None;
        };

        Self::game_pos(camera_query, screen).map(|game| Self { game, screen })
    }

    pub fn in_player_fov(&self, local_players: &LocalPlayers, players: &PlayerQuery) -> bool {
        players
            .iter()
            .find(|(player, ..)| LocalPlayer::is_local(player, local_players))
            .map(|(_, fov, ..)| fov.visible_tiles.contains_key(&self.game))
            .expect("No local player!")
    }

    fn game_pos(camera_query: &CameraQuery, screen_pos: Vec2) -> Option<IVec2> {
        let (camera, camera_transform) = camera_query.single();

        camera
            .viewport_to_world_2d(camera_transform, screen_pos)
            .map(|game| game.round().as_ivec2())
            .ok()
    }
}
