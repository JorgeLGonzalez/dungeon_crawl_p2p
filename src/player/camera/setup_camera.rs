use super::PlayerCamera;
use crate::resources::config;
use bevy::{
    prelude::*,
    render::{camera::ScalingMode, view::RenderLayers},
};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        PlayerCamera,
        Camera2d,
        Camera {
            order: 0,
            ..default()
        },
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: config::VIEWPORT_HEIGHT,
            },
            scale: config::CAMERA_SCALE,
            ..OrthographicProjection::default_2d()
        }),
        RenderLayers::layer(config::CAMERA_RENDER_LAYER),
    ));
}
