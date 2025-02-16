use super::*;
use crate::{hud::components::HudCamera, player::PlayerCamera, prelude::*};

pub type GameCameraQuery<'w, 's, 'c, 't> =
    Query<'w, 's, (&'c Camera, &'t GlobalTransform), With<PlayerCamera>>;

pub type HudCameraQuery<'w, 's, 'c, 't> =
    Query<'w, 's, (&'c Camera, &'t GlobalTransform), With<HudCamera>>;

pub struct TooltipShower {
    game_pos: Vec2,
    target_entity: Entity,
    text: String,
}

impl TooltipShower {
    pub fn new(info: &TooltipDisplayInfo) -> Self {
        Self {
            game_pos: info.game_pos,
            target_entity: info.target_entity,
            text: info.text.clone(),
        }
    }

    fn show_inner(&self, bottom: Val, left: Val, tooltip_ui: &mut TooltipUIMutQuery) {
        let (mut tooltip_node, mut tooltip_text, mut tooltip) = tooltip_ui.single_mut();

        tooltip.entity = Some(self.target_entity);
        tooltip_text.0 = self.text.clone();

        tooltip_node.display = Display::Block;
        tooltip_node.bottom = bottom;
        tooltip_node.left = left;
    }
}

impl TooltipShower {
    /// Show the tooltip at the given mouse position (converted from screen to HUD
    /// coordinates)
    pub fn show(
        &self,
        tooltip_ui: &mut TooltipUIMutQuery,
        game_camera: &GameCameraQuery,
        hud_camera_query: &HudCameraQuery,
    ) {
        let game_pos = self.game_pos;
        let (game_camera, game_transform) = game_camera.single();
        let viewport_pos = game_camera
            .world_to_viewport(game_transform, game_pos.extend(0.))
            .expect("Inconceivable!");
        let (hud_camera, hud_transform) = hud_camera_query.single();
        let ui_pos = hud_camera
            .viewport_to_world_2d(hud_transform, viewport_pos)
            .expect("Inconceivable!");

        let Vec2 { x, y } = ui_pos;

        self.show_inner(Val::Px(y), Val::Px(x), tooltip_ui);
    }
}
