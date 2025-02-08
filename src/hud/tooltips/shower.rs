use super::queries::{HudCameraQuery, TooltipUIQuery};
use bevy::prelude::*;

pub enum Position {
    Mouse(Vec2),
    Player,
}

pub struct TooltipShower {
    pos: Position,
    target_entity: Entity,
    text: String,
}

impl TooltipShower {
    pub fn new(pos: Position, target_entity: Entity, text: String) -> Self {
        Self {
            pos,
            target_entity,
            text,
        }
    }

    /// Show the tooltip at the mouse position and associate it with the target entity.
    pub fn show(&self, hud_camera_query: &HudCameraQuery, tooltip_ui: &mut TooltipUIQuery) {
        let (mut tooltip_node, mut tooltip_text, mut tooltip) = tooltip_ui.single_mut();

        tooltip_node.display = Display::Block;

        tooltip_text.0 = self.text.clone();
        tooltip.entity = Some(self.target_entity);

        let (left, bottom) = if let Position::Mouse(mouse_pos) = self.pos {
            let (hud_camera, hud_transform) = hud_camera_query.single();
            let ui_pos = hud_camera
                .viewport_to_world_2d(hud_transform, mouse_pos)
                .expect("Inconceivable!");

            (Val::Px(ui_pos.x), Val::Px(ui_pos.y))
        } else {
            (Val::Percent(50.0), Val::Percent(50.0))
        };

        tooltip_node.left = left;
        tooltip_node.bottom = bottom;
    }
}
