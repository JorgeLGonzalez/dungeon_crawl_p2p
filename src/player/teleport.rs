use super::Player;
use crate::prelude::*;

/// When the player enters a new level, we need to teleport them to their
/// new (randomly chosen) starting position (and clear their FOV).
/// This is only relevant when changing levels, not when initially spawning players.
/// Note that BOTH players are teleported regardless of which player took the
/// exit stairs. We do this to keep things simple, as otherwise we would need to
/// keep entities for multiple levels in sync between the two players. So lets
/// pretend both players are entangled so one player always drags the slowpoke onto
/// the next level.
pub fn teleport_players(
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
