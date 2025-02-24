use super::*;
use crate::{player::PlayerId, prelude::*};
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
        .set_center()
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
        let quadrant = DungeonCorner::random(rng);

        let player0_pos = self.determine_player_pos(quadrant);
        self.map.player_starting_positions.push(player0_pos);

        if config::GAME_MODE != GameMode::SinglePlayer {
            let player1_pos = self.determine_player_pos(quadrant.opposite());
            self.map.player_starting_positions.push(player1_pos);
        }

        self
    }

    /// Determines the starting position for the player in the given dungeon quadrant.
    /// Ensure player can reach dungeon center, creating a tunnel if necessary.
    fn determine_player_pos(&mut self, quadrant: DungeonCorner) -> DungeonPosition {
        let radius = 1;
        let pos = self.map.find_nearest_floor_tile(quadrant.pos(), radius);

        let player_id = self.map.player_starting_positions.len();

        match AStarPathFinder::find(pos, self.map.center, &self.map) {
            PathFindingResult::PathLength(path_len) => {
                info!("Path from player {player_id} to center has length {path_len}",);
            }
            PathFindingResult::ClosestPos(closest_pos) => self.tunnel(player_id, closest_pos),
        }

        pos
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

    /// Reset the map center to the floor tile nearest the absolute center.
    fn set_center(mut self) -> Self {
        self.map.center = self.map.find_nearest_floor_tile(self.map.center, 1);

        self
    }

    /// Tunnel from the player segment to the dungeon center segment.
    fn tunnel(&mut self, player_id: PlayerId, player_side: DungeonPosition) {
        warn!("No path found from player {player_id} to center.");

        let PathFindingResult::ClosestPos(center_side) =
            AStarPathFinder::find(self.map.center, player_side, &self.map)
        else {
            unreachable!()
        };

        Tunneler::tunnel(&mut self.map, player_side, center_side)
    }
}
