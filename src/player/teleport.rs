use super::Player;
use crate::prelude::*;

/// When the player enters a new level, we need to transport them to their
/// new starting position (and clear their FOV)
/// This is only relevant when changing levels, not when initially spawning players.
pub fn transport_players(
    dungeon: Res<DungeonMap>,
    mut players: Query<(&Player, &mut FieldOfView, &mut Transform), With<Player>>,
) {
    for (player, mut fov, mut transform) in players.iter_mut() {
        fov.visible_tiles.clear();
        let player_pos = dungeon.player_starting_positions[player.id];
        transform.translation = player_pos.to_vec3(config::PLAYER_Z_LAYER);
        info!("Transported player {} to {player_pos}", player.id);
    }
}
