use super::player::{LocalPlayer, PlayersQuery};
use crate::components::{HealthBar, Player};
use crate::resources::config;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_ggrs::LocalPlayers;

pub fn move_camera_and_hud(
    local_players: Res<LocalPlayers>,
    players: PlayersQuery,
    mut cameras: Query<&mut Transform, (With<Camera>, Without<Player>, Without<HealthBar>)>,
    mut health_bars: Query<
        (&mut Transform, &mut Sprite),
        (With<HealthBar>, Without<Camera>, Without<Player>),
    >,
) {
    let player = LocalPlayer::new(&local_players, &players);

    let mut camera = cameras.single_mut();
    camera.translation.x = player.pos.x;
    camera.translation.y = player.pos.y;

    let (mut health_bar, mut bar_sprite) = health_bars.single_mut();
    health_bar.translation.y = player.pos.y + HEALTH_BAR_OFFSET;
    health_bar.translation.x = player.pos.x;

    if let Some(size) = bar_sprite.custom_size.as_mut() {
        size.x = player.health as f32;
    }
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

const HEALTH_BAR_OFFSET: f32 = config::VIEWPORT_HEIGHT * config::CAMERA_SCALE / 2. - 0.5;
