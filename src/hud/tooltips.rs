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
    let mut ctrl = TooltipController::new(&camera_query, &mut cursor_events, &tooltip_ui, &windows);
    let result = ctrl.determine(&local_players, &players, &tooltip_entities, &mut tooltip_ui);

    match result {
        TooltipResult::Hide => ctrl.hide(&mut tooltip_ui),
        TooltipResult::NoChange => {}
        TooltipResult::Show => ctrl.show(&hud_camera_query, &mut tooltip_ui),
    }
}

enum TooltipResult {
    Hide,
    NoChange,
    Show,
}

struct TooltipController {
    game_pos: Option<Vec2>,
    is_shown: bool,
    mouse_moved: bool,
    mouse_pos: Option<Vec2>,
    target_entity: Option<Entity>,
    text: String,
}

impl TooltipController {
    pub fn new(
        camera_query: &Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
        cursor_events: &mut EventReader<CursorMoved>,
        tooltip_ui: &Query<(&mut Node, &mut Text, &mut TooltipUI)>,
        windows: &Query<&Window, With<PrimaryWindow>>,
    ) -> Self {
        let (.., tooltip) = tooltip_ui.single();
        let is_shown = tooltip.entity.is_some();

        let mouse_moved = !cursor_events.is_empty();
        cursor_events.clear();

        let mouse_pos = windows.single().cursor_position();
        let game_pos = Self::to_game_pos(mouse_pos, camera_query);

        Self {
            game_pos,
            is_shown,
            mouse_moved,
            mouse_pos,
            target_entity: None,
            text: String::new(),
        }
    }

    pub fn determine(
        &mut self,
        local_players: &LocalPlayers,
        players: &Query<(&Player, &FieldOfView)>,
        tooltip_entities: &Query<(Entity, &TooltipLabel, &Transform)>,
        tooltip_ui: &mut Query<(&mut Node, &mut Text, &mut TooltipUI)>,
    ) -> TooltipResult {
        if self.is_shown {
            if self.game_pos.is_none() || !self.in_player_fov(local_players, players) {
                return TooltipResult::Hide;
            }

            if self.still_on_entity(tooltip_entities, tooltip_ui) {
                return TooltipResult::NoChange;
            } // else continue below
        }
        // tooltip hidden or shown and no longer over entity

        if self.game_pos.is_none()
            || !self.in_player_fov(local_players, players)
            || !self.mouse_moved
        {
            return TooltipResult::NoChange;
        }

        if let Some((tooltip_entity, tooltip_label)) = self.find_entity_to_tooltip(tooltip_entities)
        {
            self.target_entity = Some(tooltip_entity);
            self.text = tooltip_label;
            return TooltipResult::Show;
        } else {
            if self.is_shown {
                return TooltipResult::Hide;
            }
            return TooltipResult::NoChange;
        }
    }

    pub fn hide(&self, tooltip_ui: &mut Query<(&mut Node, &mut Text, &mut TooltipUI)>) {
        let (mut tooltip_node, mut tooltip_text, mut tooltip) = tooltip_ui.single_mut();
        tooltip_node.display = Display::None;
        tooltip_text.0 = String::new();
        tooltip.entity = None;
    }

    pub fn show(
        &self,
        hud_camera_query: &Query<(&Camera, &GlobalTransform), With<HudCamera>>,
        tooltip_ui: &mut Query<(&mut Node, &mut Text, &mut TooltipUI)>,
    ) {
        let target_entity = self.target_entity.expect("Target entity not set!");

        let (mut tooltip_node, mut tooltip_text, mut tooltip) = tooltip_ui.single_mut();

        tooltip_node.display = Display::Block;
        tooltip_text.0 = self.text.clone();
        tooltip.entity = Some(target_entity);

        let mouse_pos = self.mouse_pos.expect("MousePos not set!");
        let (hud_camera, hud_transform) = hud_camera_query.single();
        let ui_pos = hud_camera
            .viewport_to_world_2d(hud_transform, mouse_pos)
            .expect("Inconceivable!");

        tooltip_node.left = Val::Px(ui_pos.x);
        tooltip_node.bottom = Val::Px(ui_pos.y);
    }

    fn find_entity_to_tooltip(
        &self,
        tooltip_entities: &Query<(Entity, &TooltipLabel, &Transform)>,
    ) -> Option<(Entity, String)> {
        tooltip_entities
            .iter()
            .find(|(.., transform)| self.hit_test(transform))
            .map(|(entity, label, _)| (entity, label.0.clone()))
    }

    fn hit_test(&self, transform: &Transform) -> bool {
        let Some(point) = self.game_pos else {
            return false;
        };

        let tile_pos = transform.translation.truncate();
        let min = tile_pos - 0.5;
        let max = tile_pos + 0.5;

        point.x > min.x && point.x < max.x && point.y > min.y && point.y < max.y
    }

    fn in_player_fov(
        &self,
        local_players: &LocalPlayers,
        players: &Query<(&Player, &FieldOfView)>,
    ) -> bool {
        let Some(game_pos) = self.game_pos else {
            return false;
        };

        players
            .iter()
            .find(|(player, _)| LocalPlayer::is_local(player, &local_players))
            .map(|(_, fov)| fov)
            .expect("No local player to follow!")
            .visible_tiles
            .contains_key(&game_pos.as_ivec2())
    }

    fn still_on_entity(
        &self,
        tooltip_entities: &Query<(Entity, &TooltipLabel, &Transform)>,
        tooltip_ui: &mut Query<(&mut Node, &mut Text, &mut TooltipUI)>,
    ) -> bool {
        let (.., tooltip) = tooltip_ui.single();
        let tooltip_entity = tooltip.entity.expect("No active entity!");

        let (.., transform) = tooltip_entities
            .get(tooltip_entity)
            .expect("Inconceivable!");

        self.hit_test(transform)
    }

    fn to_game_pos(
        mouse_pos: Option<Vec2>,
        camera_query: &Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
    ) -> Option<Vec2> {
        let (camera, camera_transform) = camera_query.single();

        mouse_pos.map(|pos| {
            camera
                .viewport_to_world_2d(camera_transform, pos)
                .expect("Inconceivable!")
        })
    }
}
