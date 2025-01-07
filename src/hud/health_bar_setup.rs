use super::{HealthBar, HealthPointsText};
use crate::resources::{
    assets::FontAssets,
    config::{self},
};
use bevy::{
    color::palettes::css::{GRAY, WHITE},
    prelude::*,
    render::view::RenderLayers,
};

pub fn setup_health_bar(mut commands: Commands, font_assets: Res<FontAssets>) {
    let display = Display::Flex;
    let text_color = TextColor(WHITE.into());
    let text_font = TextFont {
        font: font_assets.fira_sans_bold.clone(),
        font_size: 20.,
        ..default()
    };
    let text_node = Node {
        display,
        ..default()
    };

    commands
        // HUD container (top bar)
        .spawn((
            Node {
                display,
                position_type: PositionType::Absolute,
                overflow: Overflow::clip(),
                width: Val::Percent(100.),
                height: Val::Px(40.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.5)),
            RenderLayers::layer(config::HUD_CAMERA_RENDER_LAYER),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Health"),
                text_color.clone(),
                text_font.clone(),
                text_node.clone(),
            ));

            // health bar itself. background rect with green rect child on top
            parent
                .spawn((
                    Node {
                        display,
                        height: Val::Px(20.),
                        margin: UiRect::horizontal(Val::Px(10.)),
                        width: Val::Px(200.),
                        ..default()
                    },
                    BackgroundColor(GRAY.into()),
                ))
                .with_child((
                    HealthBar,
                    Node {
                        display,
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0., 0.7, 0.)),
                ));

            parent.spawn((
                HealthPointsText,
                Text::new(format!(
                    "{}/{}",
                    config::PLAYER_HEALTH_MAX,
                    config::PLAYER_HEALTH_MAX
                )),
                text_color,
                text_font,
                text_node,
            ));
        });
}
