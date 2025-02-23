use super::*;
use crate::prelude::*;
use rand::prelude::*;

pub struct CellAutomataBuilder {
    map: DungeonMap,
}

impl CellAutomataBuilder {
    pub fn build(rng: &mut RandomGenerator) -> DungeonMap {
        info!("Building cellular automata dungeon.");

        Self {
            map: DungeonMap::new(),
        }
        .randomize_tiles(rng)
        .grow_cells()
        .add_player_starting_positions(rng)
        .add_items(rng)
        .add_monster_starting_positions(rng)
        .map
    }

    fn add_items(mut self, rng: &mut RandomGenerator) -> Self {
        self.map.item_positions = self
            .map
            .spawnable_positions()
            .choose_multiple(rng, NUM_ITEMS);

        self
    }

    fn add_monster_starting_positions(mut self, rng: &mut RandomGenerator) -> Self {
        self.map.monster_starting_positions = self
            .map
            .spawnable_positions()
            .choose_multiple(rng, NUM_MONSTERS);

        self
    }

    /// Randomly assign player starting positions to opposite corners of the
    /// dungeon.
    fn add_player_starting_positions(mut self, rng: &mut RandomGenerator) -> Self {
        let corner = match rng.gen_range(0..4) {
            0 => DungeonCorner::BottomLeft,
            1 => DungeonCorner::BottomRight,
            2 => DungeonCorner::TopLeft,
            3 => DungeonCorner::TopRight,
            _ => unreachable!(),
        };

        let radius = 1;
        let pos = self.find_nearest_floor_tile(corner.pos(), radius);
        self.map.player_starting_positions.push(pos);

        let center = self.find_nearest_floor_tile(DungeonPosition::new(0, 0), 1);

        match AStarPathFinder::find(pos, center, &self.map) {
            PathFindingResult::PathLength(path_len) => {
                info!("Path from player 0 to center has length {path_len}");
            }
            PathFindingResult::ClosestPos(closest_pos) => {
                warn!("No path found from player 0 to center.");
                match AStarPathFinder::find(center, closest_pos, &self.map) {
                    PathFindingResult::ClosestPos(pos2) => self.tunnel(closest_pos, pos2),
                    _ => unreachable!(),
                }
            }
        }

        if config::GAME_MODE != GameMode::SinglePlayer {
            let pos = self.find_nearest_floor_tile(corner.opposite().pos(), radius);
            self.map.player_starting_positions.push(pos);

            match AStarPathFinder::find(pos, center, &self.map) {
                PathFindingResult::PathLength(path_len) => {
                    info!("Path from player 1 to center has length {path_len}");
                }
                PathFindingResult::ClosestPos(closest_pos) => {
                    warn!("No path found from player 1 to center.");
                    match AStarPathFinder::find(center, closest_pos, &self.map) {
                        PathFindingResult::ClosestPos(pos2) => self.tunnel(closest_pos, pos2),
                        _ => unreachable!(),
                    }
                }
            }
        }

        self
    }

    /// Find the nearest floor tile to the given origin, within the given radius.
    /// If no floor tile is found within the radius, recursively search with an
    /// increased radius.
    fn find_nearest_floor_tile(&self, origin: DungeonPosition, radius: isize) -> DungeonPosition {
        assert!(radius > 0 && radius < 10);
        if radius == 1 && self.map.get_tile_type(&origin) == TileType::Floor {
            return origin;
        }

        origin
            .perimeter(radius)
            .filter(|pos| self.map.is_valid_position(pos))
            .find(|pos| self.map.get_tile_type(pos) == TileType::Floor)
            .unwrap_or_else(|| self.find_nearest_floor_tile(origin, radius + 1))
    }

    /// Smooth out the randomly assigned tiles by converting tiles to floor unless
    /// surrounded by too many walls (or no walls at all).
    fn grow_cells(mut self) -> Self {
        self.map = CellGrower::grow(self.map);

        self
    }

    /// Randomly assign tiles within the map, slightly favoring floors.
    /// Map perimeter is left as walls.
    fn randomize_tiles(mut self, rng: &mut RandomGenerator) -> Self {
        self.map
            .tiles()
            .filter(|t| !t.pos.at_perimeter())
            .filter(|t| rng.gen_range(0..100) >= 55 && t.tile_type == TileType::Wall)
            .map(|t| t.pos)
            .collect::<Vec<_>>()
            .iter()
            .for_each(|pos| {
                self.map.set_tile_type(&pos, TileType::Floor);
            });

        self
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
