use crate::components::{Health, HealthBar, Player};
use crate::resources::config::{self, GameMode};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_ggrs::LocalPlayers;

pub fn move_camera_and_hud(
    local_players: Res<LocalPlayers>,
    players: Query<(&Player, &Transform, &Health), (Without<Camera>, Without<HealthBar>)>,
    mut cameras: Query<&mut Transform, With<Camera>>,
    mut health_bars: Query<(&mut Transform, &mut Sprite), (With<HealthBar>, Without<Camera>)>,
) {
    let (player_pos, health) = players
        .iter()
        .find(|(p, ..)| {
            config::GAME_MODE == GameMode::SinglePlayer || local_players.0.contains(&p.id)
        })
        .map(|(_, t, h)| (t.translation, h.current))
        .expect("No local player to follow!");

    let mut camera = cameras.single_mut();
    camera.translation.x = player_pos.x;
    camera.translation.y = player_pos.y;

    let (mut health_bar, mut bar_sprite) = health_bars.single_mut();
    health_bar.translation.y = player_pos.y + HEALTH_BAR_OFFSET;
    health_bar.translation.x = player_pos.x;

    if let Some(size) = bar_sprite.custom_size.as_mut() {
        size.x = health as f32;
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
