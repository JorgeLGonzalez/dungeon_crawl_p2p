use super::{config, FontAssets, TooltipUI};
use bevy::{prelude::*, render::view::RenderLayers};

pub fn spawn_tooltip(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands.spawn((
        TooltipUI::default(),
        Text::new(String::new()),
        BackgroundColor(config::BACKGROUND_COLOR.into()),
        TextColor(config::TEXT_COLOR.into()),
        TextFont {
            font: font_assets.hud_font.clone(),
            font_size: 16.,
            ..default()
        },
        Node {
            display: Display::None,
            position_type: PositionType::Absolute,
            ..default()
        },
        RenderLayers::layer(config::CAMERA_RENDER_LAYER),
    ));
}
