use crate::components::Player;
use bevy::prelude::{Camera, Query, Res, Transform, With, Without};
use bevy_ggrs::LocalPlayers;

pub fn camera_follow(
    local_players: Res<LocalPlayers>,
    players: Query<(&Player, &Transform)>,
    mut cameras: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player_pos = players
        .iter()
        .find(|(p, _)| local_players.0.contains(&p.id))
        .map(|(_, t)| t.translation)
        .expect("No local player to follow!");

    let mut camera = cameras.single_mut();
    camera.translation.x = player_pos.x;
    camera.translation.y = player_pos.y;
}
