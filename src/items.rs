mod components;
mod despawn_items;
mod plugin;
mod spawn_items;

pub use components::{Grabbable, MagicItem, MagicItemTemplate, Weapon};
pub use plugin::ItemsPlugin;

use components::MagicItemBundle;
use despawn_items::despawn_items;
use spawn_items::spawn_items;
