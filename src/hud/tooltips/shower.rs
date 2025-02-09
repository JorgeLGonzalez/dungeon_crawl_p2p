use super::queries::{HudCameraQuery, TooltipUIQuery};
use bevy::prelude::*;

pub trait TooltipPosition {}
pub struct MouseTooltip(pub Vec2);
impl TooltipPosition for MouseTooltip {}
pub struct PlayerTooltip;
impl TooltipPosition for PlayerTooltip {}

pub struct TooltipDisplayInfo<T: TooltipPosition> {
    pub kind: T,
    pub target_entity: Entity,
    pub text: String,
}

impl<T: TooltipPosition> TooltipDisplayInfo<T> {
    pub fn new(kind: T, target_entity: Entity, text: String) -> Self {
        Self {
            kind,
            target_entity,
            text,
        }
    }
}

pub struct TooltipShower<T: TooltipPosition> {
    kind: T,
    target_entity: Entity,
    text: String,
}

impl<T: TooltipPosition> TooltipShower<T> {
    pub fn new(info: TooltipDisplayInfo<T>) -> Self {
        Self {
            kind: info.kind,
            target_entity: info.target_entity,
            text: info.text,
        }
    }

    fn show_inner(&self, bottom: Val, left: Val, tooltip_ui: &mut TooltipUIQuery) {
        let (mut tooltip_node, mut tooltip_text, mut tooltip) = tooltip_ui.single_mut();

        tooltip.entity = Some(self.target_entity);
        tooltip_text.0 = self.text.clone();

        tooltip_node.display = Display::Block;
        tooltip_node.bottom = bottom;
        tooltip_node.left = left;
    }
}

impl TooltipShower<MouseTooltip> {
    pub fn show(&self, hud_camera_query: &HudCameraQuery, tooltip_ui: &mut TooltipUIQuery) {
        let mouse_pos = self.kind.0;

        let (hud_camera, hud_transform) = hud_camera_query.single();
        let ui_pos = hud_camera
            .viewport_to_world_2d(hud_transform, mouse_pos)
            .expect("Inconceivable!");

        self.show_inner(Val::Px(ui_pos.y), Val::Px(ui_pos.x), tooltip_ui);
    }
}

impl TooltipShower<PlayerTooltip> {
    pub fn show(&self, tooltip_ui: &mut TooltipUIQuery) {
        self.show_inner(Val::Percent(50.0), Val::Percent(50.0), tooltip_ui);
    }
}
