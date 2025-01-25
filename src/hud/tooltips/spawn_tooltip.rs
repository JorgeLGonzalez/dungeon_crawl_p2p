use super::{FontAssets, TooltipUI};
use crate::config;
use bevy::{color::palettes::css::WHITE, prelude::*, render::view::RenderLayers};

pub fn spawn_tooltip(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands.spawn((
        TooltipUI::default(),
        Text::new(String::new()),
        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.5)),
        TextColor(WHITE.into()),
        TextFont {
            font: font_assets.fira_sans_bold.clone(),
            font_size: 16.,
            ..default()
        },
        Node {
            display: Display::None,
            position_type: PositionType::Absolute,
            ..default()
        },
        RenderLayers::layer(config::HUD_CAMERA_RENDER_LAYER),
    ));
}
