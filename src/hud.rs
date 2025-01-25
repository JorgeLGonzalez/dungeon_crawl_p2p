mod assets;
mod camera_setup;
mod components;
mod health_bar;
mod health_bar_setup;
mod inventory;
mod plugin;
mod tooltips {
    mod determiner;
    mod determiner_builder;
    mod hider;
    mod queries;
    mod shower;
    mod spawn_tooltip;
    mod tooltip;

    pub use spawn_tooltip::spawn_tooltip;
    pub use tooltip::tooltip;

    use super::*;
}

pub use assets::FontAssets;
pub use components::TooltipLabel;
pub use plugin::{HudCoreSet, HudPlugin};

use camera_setup::setup_camera;
use components::*;
use health_bar::health_bar;
use health_bar_setup::setup_health_bar;
use inventory::{spawn_inventory_ui, update_inventory};
use tooltips::{spawn_tooltip, tooltip};
