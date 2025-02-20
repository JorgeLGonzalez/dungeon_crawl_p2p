use super::*;
use crate::{hud::components::HudCamera, player::PlayerCamera, prelude::*};

pub type GameCameraQuery<'w, 's, 'c, 't> =
    Query<'w, 's, (&'c Camera, &'t GlobalTransform), With<PlayerCamera>>;

pub type HudCameraQuery<'w, 's, 'c, 't> =
    Query<'w, 's, (&'c Camera, &'t GlobalTransform), With<HudCamera>>;

pub struct TooltipShower {
    target_entity: Entity,
    text: String,
    ui_pos: Vec2,
}

impl TooltipShower {
    /// Create the shower from the given display info. Convert the game position
    /// to the corresponding HUD UI position.
    pub fn new(
        info: &TooltipDisplayInfo,
        game_camera: &GameCameraQuery,
        hud_camera_query: &HudCameraQuery,
    ) -> Self {
        let game_pos = info.game_pos;
        let (game_camera, game_transform) = game_camera.single();
        let viewport_pos = game_camera
            .world_to_viewport(game_transform, game_pos.extend(0.))
            .expect("Inconceivable!");
        let (hud_camera, hud_transform) = hud_camera_query.single();
        let ui_pos = hud_camera
            .viewport_to_world_2d(hud_transform, viewport_pos)
            .expect("Inconceivable!");

        Self {
            target_entity: info.target_entity,
            text: info.text.clone(),
            ui_pos,
        }
    }

    /// Show the tooltip at the relevant game position
    pub fn show(&self, tooltip_ui: &mut TooltipUIMutQuery) {
        let (mut tooltip_node, mut tooltip_text, mut tooltip) = tooltip_ui.single_mut();

        tooltip.entity = Some(self.target_entity);
        tooltip_text.0 = self.text.clone();

        let Vec2 { x, y } = self.ui_pos;
        tooltip_node.bottom = Val::Px(y);
        tooltip_node.left = Val::Px(x);
        tooltip_node.display = Display::Block;
    }
}
