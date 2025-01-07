mod camera_setup;
mod health_bar;
mod health_bar_setup;
mod tooltips;

pub use camera_setup::setup_camera;
pub use health_bar::health_bar;
pub use health_bar_setup::setup_health_bar;
pub use tooltips::{spawn_tooltip, tooltip};

use bevy::prelude::*;

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct HealthPointsText;

#[derive(Component)]
pub struct HudCamera;

/// The text or label to use for an entity's (e.g. monster) tooltip
#[derive(Component)]
pub struct TooltipLabel(pub String);

/// The single UI entity that is used to display the tooltip label as part of the
/// HUD UI
#[derive(Component)]
pub struct TooltipUI;
