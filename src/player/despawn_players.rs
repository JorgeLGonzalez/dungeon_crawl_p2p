use super::Player;
use bevy::prelude::*;

pub fn despawn_players(mut commands: Commands, players: Query<Entity, With<Player>>) {
    // TODO: keep inventory
    players
        .iter()
        .for_each(|e| commands.entity(e).despawn_recursive());
}
