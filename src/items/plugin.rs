use super::spawn_items::spawn_items;
use crate::{dungeon::SpawnDungeonSet, prelude::*};
use bevy::prelude::*;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            spawn_items.after(SpawnDungeonSet),
        );
    }
}
