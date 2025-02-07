use super::{config, FontAssets, InventoryPanel, InventoryTitle};
use crate::prelude::*;
use bevy::render::view::RenderLayers;

/// The inventory UI panel shown on top left of the screen.
pub fn spawn_inventory_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    let heading_font = TextFont {
        font: font_assets.hud_font.clone(),
        font_size: config::TEXT_SIZE,
        ..default()
    };

    commands
        .spawn((
            InventoryPanel,
            BackgroundColor(config::BACKGROUND_COLOR.into()),
            Node {
                align_items: AlignItems::Stretch,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                margin: UiRect::all(Val::Px(config::MARGIN)),
                overflow: Overflow::scroll(),
                position_type: PositionType::Absolute,
                ..default()
            },
            RenderLayers::layer(config::CAMERA_RENDER_LAYER),
            GlobalZIndex(0),
        ))
        .with_children(|parent| {
            parent.spawn((
                InventoryTitle,
                Text::new("Inventory (0)"),
                heading_font.clone(),
                GlobalZIndex(config::Z_INDEX),
            ));
        });
}
