use super::{config, HudCamera};
use bevy::{prelude::*, render::view::RenderLayers};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        HudCamera,
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
        OrthographicProjection {
            viewport_origin: Vec2::new(0., 0.),
            ..OrthographicProjection::default_2d()
        },
        RenderLayers::layer(config::CAMERA_RENDER_LAYER),
    ));
}
