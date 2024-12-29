use super::player::{LocalPlayer, PlayersQuery};
use crate::components::Player;
use crate::resources::config;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_ggrs::LocalPlayers;

pub fn move_camera(
    local_players: Res<LocalPlayers>,
    players: PlayersQuery,
    mut cameras: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player = LocalPlayer::new(&local_players, &players);

    let mut camera = cameras.single_mut();
    camera.translation.x = player.pos.x;
    camera.translation.y = player.pos.y;
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: config::VIEWPORT_HEIGHT,
            },
            scale: config::CAMERA_SCALE,
            ..OrthographicProjection::default_2d()
        }),
    ));
}
