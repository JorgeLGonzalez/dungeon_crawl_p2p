use super::MagicItem;
use bevy::prelude::*;

pub fn despawn_items(mut commands: Commands, items: Query<Entity, With<MagicItem>>) {
    items
        .iter()
        .for_each(|e| commands.entity(e).despawn_recursive());
}
