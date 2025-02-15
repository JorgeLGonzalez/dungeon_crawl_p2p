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
    mod on_monster_move;
    mod on_mouse_move;
    mod on_player_move;
    mod on_zoom;
    mod plugin;
    mod queries;
    mod spawn_tooltip;
    mod toggle;
    mod toggle_tooltip;

    pub use components::TooltipLabel;
    pub(super) use plugin::TooltipPlugin;

    use super::*;
    use components::*;
    use on_monster_move::on_monster_move;
    use on_mouse_move::on_mouse_move;
    use on_player_move::on_player_move;
    use on_zoom::on_zoom;
    use queries::*;
    use spawn_tooltip::spawn_tooltip;
    use toggle::*;
    use toggle_tooltip::toggle_tooltip;
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
use weapon::*;
