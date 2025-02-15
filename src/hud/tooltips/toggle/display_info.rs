use bevy::prelude::*;

pub trait TooltipPosition: Clone {}

#[derive(Clone, Copy, Debug)]
pub struct MouseTooltip(pub Vec2);
impl TooltipPosition for MouseTooltip {}

#[derive(Clone, Copy, Debug)]
pub struct PlayerTooltip;
impl TooltipPosition for PlayerTooltip {}

#[derive(Debug)]
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
