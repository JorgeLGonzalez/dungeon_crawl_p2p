use crate::components::Player;
use crate::resources::config;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_ggrs::LocalPlayers;

pub fn move_camera(
    local_players: Res<LocalPlayers>,
    players: Query<(&Player, &Transform)>,
    mut cameras: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player_pos = players
        .iter()
        .find(|(p, _)| local_players.0.contains(&p.id))
        .map(|(_, t)| t.translation)
        .expect("No local player to follow!");

    let mut camera = cameras.single_mut();
    camera.translation.x = player_pos.x;
    camera.translation.y = player_pos.y;
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
