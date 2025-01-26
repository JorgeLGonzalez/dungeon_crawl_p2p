use super::{spawn_items::spawn_items, Grabbable, MagicItem};
use crate::{monsters::SpawnMonstersSet, prelude::*};
use bevy::prelude::*;
use bevy_ggrs::prelude::*;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            spawn_items.after(SpawnMonstersSet),
        );

        if !game_mode(GameMode::GgrsSyncTest) {
            app.rollback_component_with_copy::<Grabbable>()
                .checksum_component_with_hash::<Grabbable>()
                .rollback_component_with_clone::<MagicItem>()
                .checksum_component_with_hash::<MagicItem>();
        }
    }
}
