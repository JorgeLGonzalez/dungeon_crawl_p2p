use super::Monster;
use bevy::prelude::*;

pub fn despawn_monsters(mut commands: Commands, monsters: Query<Entity, With<Monster>>) {
    info!("|HIGHLIGHT| Destroying current monsters");
    monsters
        .iter()
        .for_each(|e| commands.entity(e).despawn_recursive());
}
