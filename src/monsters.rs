mod components;
mod events;
mod monster_actions;
mod plugin;
mod spawn_monsters;

pub use components::{LastAction, Monster};
pub use plugin::{MonstersCoreSet, MonstersPlugin};

use monster_actions::*;
use spawn_monsters::spawn_monsters;
