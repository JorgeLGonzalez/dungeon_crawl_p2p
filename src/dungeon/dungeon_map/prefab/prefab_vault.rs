use super::{site_selector::VaultSiteSelector, *};
use crate::prelude::*;

/// Create a prefabricated vault based on a blueprint.
pub struct PrefabVault {
    blueprint: PrefabBlueprint,
    dimensions: IVec2,
}

impl PrefabVault {
    /// Create an instance based on the given blueprint in preparation for
    /// creating the vault in the dungeon.
    pub fn from(blueprint: PrefabBlueprint) -> Self {
        let dimensions = blueprint.dimensions();

        Self {
            blueprint,
            dimensions,
        }
    }

    /// Find a random, but valid location in the dungeon where to place the vault.
    /// Clear any prior content (monsters and items) and then replace the tiles
    /// with those from the vault. Finally, ensure the vault marker pos is
    /// reachable by all players, creating tunnels if necessary.
    pub fn create_in(
        &self,
        map: &mut DungeonMap,
        rng: &mut RandomGenerator,
    ) -> Option<DungeonPosition> {
        let Some(center_pos) = VaultSiteSelector::new(self.dimensions).select(map, rng) else {
            warn!("Unable to find a proper location for the prefab vault!");
            return None;
        };

        let vault = self.vault_rect(center_pos);

        map.clear_area(vault);

        let tiles = self.blueprint.tiles(vault).collect::<Vec<_>>();
        tiles.iter().for_each(|tile| {
            tile.add_to(map);
        });

        self.ensure_reachable(map, &tiles);

        info!("{:?} prefab vault created at {center_pos}.", self.blueprint);

        Some(center_pos)
    }

    /// Ensure that the vault is reachable from all player starting positions.
    fn ensure_reachable(&self, map: &mut DungeonMap, tiles: &[BlueprintTile]) {
        let key_pos = tiles
            .iter()
            .find(|t| matches!(t, BlueprintTile::KeyMarker(_)))
            .expect("Blueprint is missing a KeyMarker tile")
            .pos();

        ReachabilityEnsurer::ensure(&Searchers::from_players(map), key_pos, map);
    }

    fn vault_rect(&self, pos: DungeonPosition) -> IRect {
        IRect::from_center_size(pos.into(), self.dimensions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::dungeon_map::reachability::AStarPathFinder;

    #[test]
    fn warn_when_unable_to_locate() {
        let mut prefab = PrefabVault::from(PrefabBlueprint::Fortress);
        prefab.dimensions = IVec2::new(MAP_WIDTH as i32 - 3, MAP_HEIGHT as i32 - 3);
        let mut map = create_map();

        assert!(prefab
            .create_in(&mut map, &mut RandomGenerator::new())
            .is_none());
    }

    #[test]
    fn clear_prior_content() {
        let mut map = create_map();
        map.tiles().collect::<Vec<_>>().iter().for_each(|t| {
            map.add_one_item(ItemPosition::new(t.pos));
            map.add_one_monster(MonsterPosition::new(t.pos));
        });
        let prefab = PrefabVault::from(PrefabBlueprint::Fortress);
        let expected_item_count = prefab
            .blueprint
            .tiles(IRect::from_center_size(IVec2::ZERO, prefab.dimensions))
            .filter(|t| match *t {
                BlueprintTile::Item(_) => true,
                BlueprintTile::Map(_) => true,
                BlueprintTile::Sword(_) => true,
                _ => false,
            })
            .count();
        let expected_monster_count = prefab
            .blueprint
            .tiles(IRect::from_center_size(IVec2::ZERO, prefab.dimensions))
            .filter(|t| match *t {
                BlueprintTile::Monster(_) => true,
                BlueprintTile::Ogre(_) => true,
                _ => false,
            })
            .count();

        let center = prefab
            .create_in(&mut map, &mut RandomGenerator::new())
            .expect("no location found");

        let vault = prefab.vault_rect(center);
        let item_count = map
            .item_positions()
            .filter(|i| vault.contains(i.pos.into()))
            .count();
        assert_eq!(item_count, expected_item_count, "wrong item count");
        let monster_count = map
            .monster_starting_positions()
            .filter(|i| vault.contains(i.pos.into()))
            .count();
        assert_eq!(monster_count, expected_monster_count, "wrong monster count");
    }

    #[test]
    fn create_in() {
        let mut map = create_map();
        let prefab = PrefabVault::from(PrefabBlueprint::Fortress);
        let center = prefab
            .create_in(&mut map, &mut RandomGenerator::new())
            .expect("no location found");

        let vault = prefab.vault_rect(center);
        let tiles = prefab.blueprint.tiles(vault).collect::<Vec<_>>();
        let expected_wall_count = tiles
            .iter()
            .filter(|t| t.tile_type() == TileType::Wall)
            .count();
        let expected_floor_count = tiles.len() - expected_wall_count;

        let mut floor_count = 0;
        let mut wall_count = 0;
        for pos in iter_positions(vault) {
            let tile = map.get_tile_type(&pos);
            match tile {
                TileType::Floor => floor_count += 1,
                TileType::Wall => wall_count += 1,
            }
        }
        assert_eq!(wall_count, expected_wall_count, "wrong wall count");
        assert_eq!(floor_count, expected_floor_count, "wrong floor count");
    }

    #[test]
    fn ensure_reachable() {
        let mut map = create_map();
        let prefab = PrefabVault::from(PrefabBlueprint::Fortress);

        let vault_pos = prefab
            .create_in(&mut map, &mut RandomGenerator::new())
            .expect("no location found");

        map.player_starting_positions.iter().for_each(|&pos| {
            let path_finder = AStarPathFinder::find(pos, vault_pos, &map);
            assert!(
                path_finder.path_found(),
                "player at {pos} is unable to reach key at {}",
                vault_pos
            );
        });
    }

    fn create_map() -> DungeonMap {
        let mut map = DungeonMap::new(1);
        map.player_starting_positions
            .push(DungeonPosition::new(X_MIN + 1, Y_MIN + 1));
        map.player_starting_positions
            .push(DungeonPosition::new(X_MAX - 1, Y_MAX - 1));

        map
    }

    fn iter_positions(vault: IRect) -> impl Iterator<Item = DungeonPosition> {
        (vault.min.y..=vault.max.y).flat_map(move |y| {
            (vault.min.x..vault.max.x).map(move |x| DungeonPosition::new(x as isize, y as isize))
        })
    }
}
