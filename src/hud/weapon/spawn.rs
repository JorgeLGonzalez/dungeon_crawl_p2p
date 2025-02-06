use super::{config, FontAssets, WeaponText};
use crate::prelude::*;
use bevy::render::view::RenderLayers;

pub fn spawn_weapon_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn((
            BackgroundColor(config::BACKGROUND_COLOR.into()),
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::RowReverse,
                margin: UiRect::all(Val::Px(10.)),
                padding: UiRect::right(Val::Px(20.)),
                position_type: PositionType::Absolute,
                width: Val::Percent(100.),
                ..default()
            },
            RenderLayers::layer(config::CAMERA_RENDER_LAYER),
        ))
        .with_child((
            WeaponText,
            Text::new("Weapon: None"),
            TextFont {
                font: font_assets.hud_font.clone(),
                font_size: config::TEXT_SIZE,
                ..default()
            },
        ));
}
