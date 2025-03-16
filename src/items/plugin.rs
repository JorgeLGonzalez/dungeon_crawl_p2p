use super::{despawn_items, spawn_items, Grabbable, MagicItem, Weapon};
use crate::{monsters::SpawnMonstersSet, prelude::*};
use bevy::prelude::*;
use bevy_ggrs::prelude::*;

pub struct ItemsPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SpawnItemsSet;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            GgrsSchedule,
            (despawn_items, spawn_items)
                .in_set(SpawnItemsSet)
                .chain()
                .run_if(in_state(GameState::DungeonSpawning))
                .after(SpawnMonstersSet),
        );

        if !game_mode(GameMode::SinglePlayer) {
            app.rollback_component_with_copy::<Grabbable>()
                .checksum_component_with_hash::<Grabbable>()
                .rollback_component_with_copy::<MagicItem>()
                .checksum_component_with_hash::<MagicItem>()
                .rollback_component_with_copy::<Weapon>()
                .checksum_component_with_hash::<Weapon>();
        }
    }
}
