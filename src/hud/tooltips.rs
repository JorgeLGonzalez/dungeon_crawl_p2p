use super::{HudCamera, TooltipLabel, TooltipUI};
use crate::{
    player::PlayerCamera,
    resources::{assets::FontAssets, config},
};
use bevy::{color::palettes::css::WHITE, prelude::*, render::view::RenderLayers};

pub fn spawn_tooltip(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands.spawn((
        TooltipUI,
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

pub fn tooltip(
    mut cursor_events: EventReader<CursorMoved>,
    mut tooltip_ui: Query<(&mut Node, &mut Text), With<TooltipUI>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
    hud_camera_query: Query<(&Camera, &GlobalTransform), With<HudCamera>>,
    tooltip_labels: Query<(&TooltipLabel, &Transform)>,
) {
    let Some(event) = cursor_events.read().last() else {
        return;
    };

    // TODO only for player's FOV.
    // TODO skip if delta is too small?
    // TODO player ID tooltip

    let (camera, camera_transform) = camera_query.single();
    let game_pos = camera
        .viewport_to_world_2d(camera_transform, event.position)
        .expect("Inconceivable!");

    let (mut tooltip_node, mut tooltip_text) = tooltip_ui.single_mut();

    let Some(tooltip_label) = tooltip_labels
        .iter()
        .find(|(_, transform)| hit_test(game_pos, transform))
        .map(|(label, _)| label.0.clone())
    else {
        tooltip_node.display = Display::None;
        tooltip_text.0 = String::new();

        return;
    };

    tooltip_node.display = Display::Block;
    tooltip_text.0 = tooltip_label;

    let (hud_camera, hud_transform) = hud_camera_query.single();
    let ui_pos = hud_camera
        .viewport_to_world_2d(hud_transform, event.position)
        .expect("Inconceivable!");

    tooltip_node.left = Val::Px(ui_pos.x);
    tooltip_node.bottom = Val::Px(ui_pos.y);
}

fn hit_test(point: Vec2, transform: &Transform) -> bool {
    let tile_pos = transform.translation.truncate();
    let min = tile_pos - 0.5;
    let max = tile_pos + 0.5;

    point.x > min.x && point.x < max.x && point.y > min.y && point.y < max.y
}
