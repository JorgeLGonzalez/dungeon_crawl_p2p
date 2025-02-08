use super::queries::{HudCameraQuery, TooltipUIQuery};
use bevy::prelude::*;

pub enum Position {
    Mouse(Vec2),
    Player,
}

pub struct TooltipDisplayInfo {
    pub pos: Position,
    pub target_entity: Entity,
    pub text: String,
}

impl TooltipDisplayInfo {
    pub fn new(pos: Position, target_entity: Entity, text: String) -> Self {
        Self {
            pos,
            target_entity,
            text,
        }
    }
}

pub struct TooltipShower {
    pub pos: Position,
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

    pub fn show_on_mouse_cursor(
        &self,
        hud_camera_query: &HudCameraQuery,
        tooltip_ui: &mut TooltipUIQuery,
    ) {
        if let Position::Mouse(mouse_pos) = self.pos {
            let (hud_camera, hud_transform) = hud_camera_query.single();
            let ui_pos = hud_camera
                .viewport_to_world_2d(hud_transform, mouse_pos)
                .expect("Inconceivable!");

            self.show(Val::Px(ui_pos.y), Val::Px(ui_pos.x), tooltip_ui);
        } else {
            unreachable!()
        };
    }

    pub fn show_on_player(&self, tooltip_ui: &mut TooltipUIQuery) {
        self.show(Val::Percent(50.0), Val::Percent(50.0), tooltip_ui);
    }

    fn show(&self, bottom: Val, left: Val, tooltip_ui: &mut TooltipUIQuery) {
        let (mut tooltip_node, mut tooltip_text, mut tooltip) = tooltip_ui.single_mut();

        tooltip.entity = Some(self.target_entity);
        tooltip_text.0 = self.text.clone();

        tooltip_node.display = Display::Block;
        tooltip_node.bottom = bottom;
        tooltip_node.left = left;
    }
}
