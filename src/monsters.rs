mod components;
mod events;
mod monster_actions {
    mod attack_player;
    mod do_monsters_action;
    mod monster_action_determiner;
    mod monster_action_params;
    mod monster_move_tracker;
    mod move_monster;
    mod persist_monster_moves;
    mod update_last_action;

    pub(super) use attack_player::attack_player;
    pub(super) use do_monsters_action::do_monsters_action;
    pub(super) use monster_action_determiner::{MonsterAction, MonsterActionDeterminer};
    pub(super) use monster_action_params::*;
    pub(super) use monster_move_tracker::{MonsterMove, MonsterMoveTracker};
    pub(super) use move_monster::move_monster;
    pub(super) use persist_monster_moves::persist_monster_moves;
    pub(super) use update_last_action::update_last_action;

    use super::components::*;
    use super::events::*;
}
mod plugin;
mod spawn_monsters;

pub use components::{Monster, MonsterBundle, MonsterTemplate};
pub use plugin::{MonstersCoreSet, MonstersPlugin, SpawnMonstersSet};

use monster_actions::*;
