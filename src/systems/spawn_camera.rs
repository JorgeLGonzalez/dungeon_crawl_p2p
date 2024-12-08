use crate::resources::config;
use bevy::{
    prelude::{Camera2d, Commands, OrthographicProjection, Projection},
    render::camera::ScalingMode,
};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: config::VIEWPORT_HEIGHT,
            },
            scale: 3.,
            ..OrthographicProjection::default_2d()
        }),
    ));
}
