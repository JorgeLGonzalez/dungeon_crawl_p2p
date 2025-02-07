use super::{spawn_items::spawn_items, Grabbable, MagicItem, Weapon};
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
