use super::{config, FontAssets, LocationText};
use bevy::{prelude::*, render::view::RenderLayers};

pub fn spawn_location_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn((
            BackgroundColor(config::BACKGROUND_COLOR.into()),
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::RowReverse,
                height: Val::Auto,
                margin: UiRect::new(
                    Val::Px(config::MARGIN),
                    Val::Px(config::MARGIN),
                    Val::Auto,
                    Val::Px(0.),
                ),
                padding: UiRect::right(Val::Px(20.)),
                position_type: PositionType::Absolute,
                width: Val::Percent(100.),
                ..default()
            },
            GlobalZIndex(0),
            RenderLayers::layer(config::CAMERA_RENDER_LAYER),
        ))
        .with_child((
            LocationText,
            Text::new("(0,0)"),
            TextFont {
                font: font_assets.hud_font.clone(),
                font_size: config::TEXT_SIZE,
                ..default()
            },
            GlobalZIndex(config::Z_INDEX),
        ));
}
