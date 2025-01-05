mod health_bar;
mod spawn_health_bar;

pub use health_bar::health_bar;
pub use spawn_health_bar::setup_health_bar;

use bevy::prelude::*;

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct HealthPointsText;
