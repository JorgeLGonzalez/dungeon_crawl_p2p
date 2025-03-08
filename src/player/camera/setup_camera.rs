use super::PlayerCamera;
use crate::{config, dungeon::VIEWPORT_HEIGHT, prelude::*};
use bevy::render::{camera::ScalingMode, view::RenderLayers};

pub fn setup_camera(mut commands: Commands, camera: Query<&PlayerCamera>) {
    if !camera.is_empty() {
        // Camera already exists, no need to create a new one
        return;
    }

    commands.spawn((
        PlayerCamera,
        Camera2d,
        Camera {
            order: 0,
            ..default()
        },
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: VIEWPORT_HEIGHT,
            },
            scale: config::CAMERA_SCALE,
            ..OrthographicProjection::default_2d()
        }),
        RenderLayers::layer(config::CAMERA_RENDER_LAYER),
    ));
}
