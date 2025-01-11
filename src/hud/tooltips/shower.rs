use super::queries::{HudCameraQuery, TooltipUIQuery};
use bevy::prelude::*;

pub struct TooltipShower {
    mouse_pos: Vec2,
    target_entity: Entity,
    text: String,
}

impl TooltipShower {
    pub fn new(mouse_pos: Vec2, target_entity: Entity, text: String) -> Self {
        Self {
            mouse_pos,
            target_entity,
            text,
        }
    }

    pub fn show(&self, hud_camera_query: &HudCameraQuery, tooltip_ui: &mut TooltipUIQuery) {
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
