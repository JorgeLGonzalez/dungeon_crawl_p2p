mod camera_setup;
mod components;
mod health_bar;
mod health_bar_setup;
mod plugin;
mod tooltips;

pub use components::TooltipLabel;
pub use plugin::{HudCoreSet, HudPlugin};

use camera_setup::setup_camera;
use health_bar::health_bar;
use health_bar_setup::setup_health_bar;
use tooltips::{spawn_tooltip, tooltip};
