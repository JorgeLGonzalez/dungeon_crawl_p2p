mod assets;
mod camera_setup;
mod components;
mod config;
mod health_bar;
mod health_bar_setup;
mod inventory {
    mod components;
    mod hud_inventory_sync;
    mod spawn_inventory_ui;
    mod update_inventory;

    pub use components::{InventoryItem, InventoryItemBundle};
    pub(super) use spawn_inventory_ui::spawn_inventory_ui;
    pub(super) use update_inventory::update_inventory;

    use super::*;
    use components::*;
    use hud_inventory_sync::*;
}
mod plugin;
mod tooltips {
    mod components;
    mod determiner;
    mod determiner_builder;
    mod hider;
    mod queries;
    mod shower;
    mod spawn_tooltip;
    mod tooltip;

    pub use components::TooltipLabel;
    pub use spawn_tooltip::spawn_tooltip;
    pub use tooltip::tooltip;

    use super::*;
    use components::*;
}

pub use assets::FontAssets;
pub use inventory::{InventoryItem, InventoryItemBundle};
pub use plugin::{HudCoreSet, HudPlugin};
pub use tooltips::TooltipLabel;

use camera_setup::setup_camera;
use components::*;
use health_bar::health_bar;
use health_bar_setup::setup_health_bar;
use inventory::{spawn_inventory_ui, update_inventory};
use tooltips::{spawn_tooltip, tooltip};
