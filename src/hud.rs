mod assets;
mod camera_setup;
mod components;
mod health_bar;
mod health_bar_setup;
mod inventory;
mod plugin;
mod tooltips;

pub use assets::FontAssets;
pub use components::TooltipLabel;
pub use plugin::{HudCoreSet, HudPlugin};

use camera_setup::setup_camera;
use components::*;
use health_bar::health_bar;
use health_bar_setup::setup_health_bar;
use inventory::update_inventory;
use tooltips::{spawn_tooltip, tooltip};
