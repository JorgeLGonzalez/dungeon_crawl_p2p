mod components;
mod plugin;
mod spawn_items;

pub use components::{Grabbable, MagicItem, MagicItemTemplate, Weapon};
pub use plugin::ItemsPlugin;

use components::MagicItemBundle;
