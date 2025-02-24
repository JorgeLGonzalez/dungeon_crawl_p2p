use super::*;
use bevy::log::{info, warn};

/// Determines the starting position for the player in the dungeon.
pub(super) struct PlayerStart;

impl PlayerStart {
    /// Determines the starting position for the player in the given dungeon quadrant.
    /// Ensure player can reach dungeon center, creating a tunnel if necessary.
    pub fn determine(map: &mut DungeonMap, quadrant: DungeonCorner) -> DungeonPosition {
        let radius = 1;
        let pos = map.find_nearest_floor_tile(quadrant.pos(), radius);

        let player_id = map.player_starting_positions.len();

        match AStarPathFinder::find(pos, map.center, map) {
            PathFindingResult::PathLength(path_len) => {
                info!("Path from player {player_id} to center has length {path_len}",);
            }
            PathFindingResult::ClosestPos(closest_pos) => {
                warn!("No path found from player {player_id} to center.");
                match AStarPathFinder::find(map.center, closest_pos, map) {
                    PathFindingResult::ClosestPos(pos2) => Tunneler::tunnel(map, closest_pos, pos2),
                    _ => unreachable!(),
                }
            }
        }
        pos
    }
}
