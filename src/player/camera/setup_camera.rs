use super::PlayerCamera;
use crate::resources::config;
use bevy::{prelude::*, render::camera::ScalingMode};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        PlayerCamera,
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
