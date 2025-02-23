use super::*;
use crate::{player::PlayerId, prelude::*};

pub(super) struct PlayerStart<'a> {
    map: &'a mut DungeonMap,
    player_id: PlayerId,
}

impl<'a> PlayerStart<'a> {
    pub fn add_starting_position(
        map: &'a mut DungeonMap,
        player_id: PlayerId,
        quadrant: DungeonCorner,
    ) {
        Self { map, player_id }.determine(quadrant);
    }

    fn determine(&mut self, quadrant: DungeonCorner) {
        let radius = 1;
        let pos = self.map.find_nearest_floor_tile(quadrant.pos(), radius);
        self.map.player_starting_positions.push(pos);

        match AStarPathFinder::find(pos, self.map.center, self.map) {
            PathFindingResult::PathLength(path_len) => {
                info!(
                    "Path from player {} to center has length {path_len}",
                    self.player_id
                );
            }
            PathFindingResult::ClosestPos(closest_pos) => {
                warn!("No path found from player {} to center.", self.player_id);
                match AStarPathFinder::find(self.map.center, closest_pos, self.map) {
                    PathFindingResult::ClosestPos(pos2) => self.tunnel(closest_pos, pos2),
                    _ => unreachable!(),
                }
            }
        }
    }

    fn tunnel(&mut self, pos1: DungeonPosition, pos2: DungeonPosition) {
        info!("Tunneling from {pos1} to {pos2}");

        if pos1.x < pos2.x {
            self.tunnel_horizontally(pos1.x, pos2.x, pos1.y);
        } else if pos1.x > pos2.x {
            self.tunnel_horizontally(pos2.x, pos1.x, pos2.y);
        }

        if pos1.y < pos2.y {
            self.tunnel_vertically(pos1.y, pos2.y, pos1.x);
        } else if pos1.y > pos2.y {
            self.tunnel_vertically(pos2.y, pos1.y, pos2.x);
        }
    }

    fn tunnel_horizontally(&mut self, x1: isize, x2: isize, y: isize) {
        for x in x1..=x2 {
            let pos = DungeonPosition::new(x, y);
            if self.map.get_tile_type(&pos) == TileType::Wall {
                info!("Tunneled horizontally at {pos}");
                self.map.set_tile_type(&pos, TileType::Floor);
            }
        }
    }

    fn tunnel_vertically(&mut self, y1: isize, y2: isize, x: isize) {
        for y in y1..=y2 {
            let pos = DungeonPosition::new(x, y);
            if self.map.get_tile_type(&pos) == TileType::Wall {
                info!("Tunneled vertically at {pos}");
                self.map.set_tile_type(&pos, TileType::Floor);
            }
        }
    }
}
