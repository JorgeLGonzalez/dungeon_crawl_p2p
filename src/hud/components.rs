use bevy::prelude::*;

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct HealthPointsText;

#[derive(Component)]
pub struct HudCamera;

#[derive(Component)]
pub struct InventoryTitle;

/// The text or label to use for an entity's (e.g. monster) tooltip
#[derive(Component)]
pub struct TooltipLabel(pub String);

/// The single UI entity that is used to display the tooltip label as part of the
/// HUD UI
#[derive(Component, Default)]
pub struct TooltipUI {
    pub entity: Option<Entity>,
}
