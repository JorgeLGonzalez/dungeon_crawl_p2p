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
    let result = TooltipController::new(
        &camera_query,
        &mut cursor_events,
        &local_players,
        &players,
        &tooltip_ui,
        &windows,
    )
    .determine(&tooltip_entities, &mut tooltip_ui);

    match result {
        TooltipResult::Hide(hider) => hider.hide(&mut tooltip_ui),
        TooltipResult::NoChange => {}
        TooltipResult::Show(shower) => shower.show(&hud_camera_query, &mut tooltip_ui),
    }
}

enum TooltipResult {
    Hide(TooltipHider),
    NoChange,
    Show(TooltipShower),
}

struct TooltipController {
    game_pos: Option<Vec2>,
    in_fov: bool,
    is_shown: bool,
    mouse_moved: bool,
    mouse_pos: Option<Vec2>,
}

impl TooltipController {
    pub fn new(
        camera_query: &Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
        cursor_events: &mut EventReader<CursorMoved>,
        local_players: &LocalPlayers,
        players: &Query<(&Player, &FieldOfView)>,
        tooltip_ui: &Query<(&mut Node, &mut Text, &mut TooltipUI)>,
        windows: &Query<&Window, With<PrimaryWindow>>,
    ) -> Self {
        let (.., tooltip) = tooltip_ui.single();
        let is_shown = tooltip.entity.is_some();

        let mouse_moved = !cursor_events.is_empty();
        cursor_events.clear();

        let mouse_pos = windows.single().cursor_position();
        let game_pos = Self::to_game_pos(mouse_pos, camera_query);
        let in_fov = game_pos.is_some_and(|pos| Self::in_player_fov(pos, local_players, players));

        Self {
            game_pos,
            in_fov,
            is_shown,
            mouse_moved,
            mouse_pos,
        }
    }

    pub fn determine(
        &mut self,

        tooltip_entities: &Query<(Entity, &TooltipLabel, &Transform)>,
        tooltip_ui: &Query<(&mut Node, &mut Text, &mut TooltipUI)>,
    ) -> TooltipResult {
        let interim_result = self.try_create_shower(tooltip_entities, tooltip_ui);

        if let Some(shower) = interim_result.tooltip_shower {
            TooltipResult::Show(shower)
        } else if self.is_shown && !interim_result.still_on_entity {
            TooltipResult::Hide(TooltipHider)
        } else {
            TooltipResult::NoChange
        }
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
        game_pos: Vec2,
        local_players: &LocalPlayers,
        players: &Query<(&Player, &FieldOfView)>,
    ) -> bool {
        players
            .iter()
            .find(|(player, _)| LocalPlayer::is_local(player, &local_players))
            .map(|(_, fov)| fov)
            .expect("No local player to follow!")
            .visible_tiles
            .contains_key(&game_pos.as_ivec2())
    }

    fn try_create_shower(
        &mut self,
        tooltip_entities: &Query<(Entity, &TooltipLabel, &Transform)>,
        tooltip_ui: &Query<(&mut Node, &mut Text, &mut TooltipUI)>,
    ) -> InterimResult {
        if !self.in_fov || !self.mouse_moved {
            return InterimResult {
                still_on_entity: false,
                tooltip_shower: None,
            };
        }

        if self.is_shown && self.still_on_entity(tooltip_entities, tooltip_ui) {
            // already showing on the same entity, so no need to change
            return InterimResult {
                still_on_entity: true,
                tooltip_shower: None,
            };
        }

        // Now we get the relatively expensive test to see if mouse is over
        // a relevant entity. Either a tooltip shown but now invalid or no
        // tooltip being shown but might need to be
        if let Some((tooltip_entity, tooltip_label)) = self.find_entity_to_tooltip(tooltip_entities)
        {
            InterimResult {
                still_on_entity: false,
                tooltip_shower: Some(TooltipShower {
                    mouse_pos: self.mouse_pos.unwrap(),
                    target_entity: tooltip_entity,
                    text: tooltip_label,
                }),
            }
        } else {
            InterimResult {
                still_on_entity: false,
                tooltip_shower: None,
            }
        }
    }

    fn still_on_entity(
        &self,
        tooltip_entities: &Query<(Entity, &TooltipLabel, &Transform)>,
        tooltip_ui: &Query<(&mut Node, &mut Text, &mut TooltipUI)>,
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

struct TooltipHider;

impl TooltipHider {
    pub fn hide(&self, tooltip_ui: &mut Query<(&mut Node, &mut Text, &mut TooltipUI)>) {
        let (mut tooltip_node, mut tooltip_text, mut tooltip) = tooltip_ui.single_mut();
        tooltip_node.display = Display::None;
        tooltip_text.0 = String::new();
        tooltip.entity = None;
    }
}

struct TooltipShower {
    mouse_pos: Vec2,
    target_entity: Entity,
    text: String,
}

impl TooltipShower {
    pub fn show(
        &self,
        hud_camera_query: &Query<(&Camera, &GlobalTransform), With<HudCamera>>,
        tooltip_ui: &mut Query<(&mut Node, &mut Text, &mut TooltipUI)>,
    ) {
        let (mut tooltip_node, mut tooltip_text, mut tooltip) = tooltip_ui.single_mut();

        tooltip_node.display = Display::Block;
        tooltip_text.0 = self.text.clone();
        tooltip.entity = Some(self.target_entity);

        let (hud_camera, hud_transform) = hud_camera_query.single();
        let ui_pos = hud_camera
            .viewport_to_world_2d(hud_transform, self.mouse_pos)
            .expect("Inconceivable!");

        tooltip_node.left = Val::Px(ui_pos.x);
        tooltip_node.bottom = Val::Px(ui_pos.y);
    }
}

struct InterimResult {
    still_on_entity: bool,
    tooltip_shower: Option<TooltipShower>,
}
