mod determiner;
mod determiner_builder;
mod hider;
mod queries;
mod shower;

use super::components::TooltipUI;
use crate::assets::FontAssets;
use crate::config;
use bevy::{color::palettes::css::WHITE, prelude::*, render::view::RenderLayers};
use bevy_ggrs::LocalPlayers;
use determiner::TooltipToggleAction;
use determiner_builder::TooltipDeterminerBuilder;
use queries::{
    CameraQuery, HudCameraQuery, PlayerQuery, TooltipEntityQuery, TooltipUIQuery, WindowQuery,
};

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
/// We need to convert the mouse cursor position from window to world coordinates.
/// See README.md for more information.
pub fn tooltip(
    mut cursor_events: EventReader<CursorMoved>,
    mut tooltip_ui: TooltipUIQuery,
    camera_query: CameraQuery,
    hud_camera_query: HudCameraQuery,
    local_players: Res<LocalPlayers>,
    players: PlayerQuery,
    tooltip_entities: TooltipEntityQuery,
    windows: WindowQuery,
) {
    let toggle_action = TooltipDeterminerBuilder::new(&camera_query, &mut cursor_events, &windows)
        .local_player_fov(&local_players, &players)
        .with_tooltip_ui(&mut tooltip_ui)
        .build()
        .determine(&tooltip_entities);

    match toggle_action {
        TooltipToggleAction::Hide(hider) => hider.hide(&mut tooltip_ui),
        TooltipToggleAction::None => {}
        TooltipToggleAction::Show(shower) => shower.show(&hud_camera_query, &mut tooltip_ui),
    }
}
