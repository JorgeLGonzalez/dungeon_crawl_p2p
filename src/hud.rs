mod camera_setup;
mod health_bar;
mod health_bar_setup;

pub use camera_setup::setup_camera;
pub use health_bar::health_bar;
pub use health_bar_setup::setup_health_bar;

use bevy::prelude::*;

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct HealthPointsText;

#[derive(Component)]
pub struct HudCamera;
