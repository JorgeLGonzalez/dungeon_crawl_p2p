use super::{HudCamera, TooltipLabel, TooltipUI};
use crate::{
    components::{FieldOfView, Player},
    player::{LocalPlayer, PlayerCamera},
    resources::{assets::FontAssets, config},
};
use bevy::{
    color::palettes::css::WHITE, prelude::*, render::view::RenderLayers, window::PrimaryWindow,
};
use bevy_ggrs::LocalPlayers;

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

/// Display or hide a tooltip when hovering over an entity marked with [`TooltipLabel`].
/// The [`TooltipUI`] component marks the entity used to display the tooltip whereas
/// the [`TooltipLabel`] component marks entities that can be hovered over to display
/// a tooltip.
/// Tooltips are only displayed for the local player.
/// We need to convert the mouse cursor position from window space to world space
/// coordinates.
pub fn tooltip(
    mut cursor_events: EventReader<CursorMoved>,
    mut tooltip_ui: Query<(&mut Node, &mut Text, &mut TooltipUI)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
    hud_camera_query: Query<(&Camera, &GlobalTransform), With<HudCamera>>,
    local_players: Res<LocalPlayers>,
    players: Query<(&Player, &FieldOfView)>,
    tooltip_entities: Query<(Entity, &TooltipLabel, &Transform)>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let (mut tooltip_node, mut tooltip_text, mut tooltip) = tooltip_ui.single_mut();
    let (camera, camera_transform) = camera_query.single();

    if let Some(entity) = tooltip.entity.as_mut() {
        if let Some(cursor_position) = windows.single().cursor_position() {
            if let Ok((.., transform)) = tooltip_entities.get(*entity) {
                let game_pos = camera
                    .viewport_to_world_2d(camera_transform, cursor_position)
                    .expect("Inconceivable!");

                let is_visible = players
                    .iter()
                    .find(|(player, _)| LocalPlayer::is_local(player, &local_players))
                    .map(|(_, fov)| fov)
                    .expect("No local player to follow!")
                    .visible_tiles
                    .contains_key(&game_pos.as_ivec2());

                if !is_visible {
                    // info!("Cursor no longer in player's FOV");
                    tooltip_node.display = Display::None;
                    tooltip_text.0 = String::new();
                    tooltip.entity = None;
                    // info!("Entity no longer in player's FOV");

                    return;
                }

                if hit_test(game_pos, transform) {
                    // TODO only return if we're still in player's FOV
                    // info!("Still hovering over tooltip entity");
                    return;
                }
            } else {
                error!("No transform found for tooltip entity");
            }
        } else {
            error!("No cursor position found");
        }

        // info!("Entity moved out from under cursor");
        tooltip_node.display = Display::None;
        tooltip_text.0 = String::new();
        tooltip.entity = None;
    }

    let Some(event) = cursor_events.read().last() else {
        return;
    };

    let game_pos = camera
        .viewport_to_world_2d(camera_transform, event.position)
        .expect("Inconceivable!");

    // TODO skip if delta is too small?
    // TODO player ID tooltip

    let is_visible = players
        .iter()
        .find(|(player, _)| LocalPlayer::is_local(player, &local_players))
        .map(|(_, fov)| fov)
        .expect("No local player to follow!")
        .visible_tiles
        .contains_key(&game_pos.as_ivec2());

    if !is_visible {
        // info!("Cursor not in player's FOV");
        tooltip_node.display = Display::None;
        tooltip_text.0 = String::new();
        tooltip.entity = None;

        return;
    }

    let Some((tooltip_entity, tooltip_label)) = tooltip_entities
        .iter()
        .find(|(.., transform)| hit_test(game_pos, transform))
        .map(|(entity, label, _)| (entity, label.0.clone()))
    else {
        // info!("Cursor not over tooltip entity");
        tooltip_node.display = Display::None;
        tooltip_text.0 = String::new();
        tooltip.entity = None;

        return;
    };

    tooltip_node.display = Display::Block;
    tooltip_text.0 = tooltip_label;
    tooltip.entity = Some(tooltip_entity);

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
