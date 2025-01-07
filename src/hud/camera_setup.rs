use super::HudCamera;
use crate::resources::config;
use bevy::{prelude::*, render::view::RenderLayers};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        HudCamera,
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
        OrthographicProjection::default_2d(),
        RenderLayers::layer(config::HUD_CAMERA_RENDER_LAYER),
    ));
}
