mod assets;
mod camera_setup;
mod components;
mod config;

mod health {
    mod components;
    mod health_bar;
    mod health_bar_setup;

    pub(super) use health_bar::health_bar;
    pub(super) use health_bar_setup::setup_health_bar;

    use super::*;
    use components::*;
}

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
    mod toggle_tooltip;
    mod toggle_trigger;
    mod tooltip;

    pub use components::TooltipLabel;
    pub(super) use spawn_tooltip::spawn_tooltip;
    pub(super) use toggle_tooltip::toggle_tooltip;
    pub(super) use tooltip::tooltip;

    use super::*;
    use components::*;
    use determiner_builder::*;
    use hider::*;
    use queries::*;
    use shower::*;
    use toggle_trigger::*;
}

mod weapon {
    mod components;
    mod spawn;
    mod wield;

    pub(super) use spawn::spawn_weapon_ui;
    pub(super) use wield::wield_weapon;

    use super::*;
    use components::*;
}

pub use assets::FontAssets;
pub use inventory::{InventoryItem, InventoryItemBundle};
pub use plugin::{HudCoreSet, HudPlugin};
pub use tooltips::TooltipLabel;

use camera_setup::setup_camera;
use components::*;
use health::*;
use inventory::{spawn_inventory_ui, update_inventory};
use tooltips::{spawn_tooltip, toggle_tooltip, tooltip};
use weapon::*;
