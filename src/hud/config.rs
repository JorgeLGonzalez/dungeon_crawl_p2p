use bevy::{
    color::{palettes::css::WHITE, Srgba},
    render::view::Layer,
};

pub const CAMERA_RENDER_LAYER: Layer = 1;

pub const TOP_BAR_HEIGHT: f32 = 40.;
pub const BACKGROUND_COLOR: Srgba = Srgba::new(0.1, 0.1, 0.1, 0.5);

pub const TEXT_COLOR: Srgba = WHITE;
pub const TEXT_SIZE: f32 = 20.;
