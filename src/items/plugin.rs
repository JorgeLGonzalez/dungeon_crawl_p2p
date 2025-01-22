use super::{spawn_items::spawn_items, MagicItem};
use crate::{dungeon::SpawnDungeonSet, prelude::*};
use bevy::prelude::*;
use bevy_ggrs::prelude::*;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            spawn_items.after(SpawnDungeonSet),
        );

        if !game_mode(GameMode::GgrsSyncTest) {
            app.rollback_component_with_clone::<MagicItem>()
                .checksum_component_with_hash::<MagicItem>();
        }
    }
}
