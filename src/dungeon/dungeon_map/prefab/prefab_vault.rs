use super::{site_selector::VaultSiteSelector, *};
use crate::prelude::*;

pub struct PrefabVault {
    blueprint: PrefabBlueprint,
    key_pos: DungeonPosition,
    dimensions: IVec2,
}

impl PrefabVault {
    pub fn new(blueprint: PrefabBlueprint) -> Self {
        let dimensions = blueprint.dimensions();

        Self {
            blueprint,
            key_pos: DungeonPosition::new(0, 0),
            dimensions,
        }
    }

    pub fn create_at(&mut self, center: DungeonPosition, map: &mut DungeonMap) {
        let vault = self.vault_rect(center);
        self.clear_content(vault, map);
        let tiles = self.blueprint.tiles(vault).collect::<Vec<_>>();
        tiles.iter().for_each(|tile| {
            tile.add_to(map);
        });

        self.key_pos = tiles
            .iter()
            .find(|t| matches!(t, BlueprintTile::KeyMarker(_)))
            .expect("Blueprint is missing a KeyMarker tile")
            .pos();
        ReachabilityEnsurer::ensure(&Searchers::from_players(map), self.key_pos, map);

        info!("{:?} prefab vault created at {center}.", self.blueprint);
    }

    pub fn determine_location(
        &self,
        map: &DungeonMap,
        rng: &mut RandomGenerator,
    ) -> Option<DungeonPosition> {
        VaultSiteSelector::new(self.dimensions)
            .select(map, rng)
            .map(|pos| {
                info!(
                    "Selected location for {:?} prefab vault: {pos}.",
                    self.blueprint
                );
                pos
            })
    }

    /// Remove any monsters or items slated for the tiles encompassed by the vault.
    fn clear_content(&self, vault: IRect, map: &mut DungeonMap) {
        map.item_positions
            .retain(|&pos| !vault.contains(pos.into()));
        map.monster_starting_positions
            .retain(|&pos| !vault.contains(pos.into()));
    }

    // TODO remove
    fn vault_rect(&self, pos: DungeonPosition) -> IRect {
        IRect::from_center_size(pos.into(), self.dimensions)
    }
}

#[cfg(test)]
mod tests {
    use super::{reachability::AStarPathFinder, *};
    use crate::{
        items::{MagicItem, Weapon},
        monsters::Monster,
    };
    use bevy::utils::hashbrown::HashSet;

    #[test]
    fn new() {
        let prefab = PrefabVault::new(PrefabBlueprint::Fortress);

        assert_eq!(prefab.dimensions.x, 12, "wrong width");
        assert_eq!(prefab.dimensions.y, 11, "wrong height");
    }

    #[test]
    fn create_at() {
        let mut map = create_map();
        let pos = map.center;
        let mut prefab = PrefabVault::new(PrefabBlueprint::Fortress);

        prefab.create_at(pos, &mut map);

        let blueprint = prefab
            .blueprint
            .blueprint()
            .chars()
            .filter(|c| *c != '\n' && *c != '\r')
            .collect::<String>();
        assert_eq!(blueprint.len(), 12 * 11, "wrong blueprint length");
        let expected_floor_count = blueprint.chars().filter(|c| *c != '#').count();
        let expected_wall_count = blueprint.chars().filter(|c| *c == '#').count();
        let vault = prefab.vault_rect(pos);

        let mut floor_count = 0;
        let mut wall_count = 0;
        for y in vault.min.y..=vault.max.y {
            for x in vault.min.x..vault.max.x {
                let tile = map.get_tile_type(&DungeonPosition::new(x as isize, y as isize));
                match tile {
                    TileType::Floor => floor_count += 1,
                    TileType::Wall => wall_count += 1,
                    _ => unreachable!(),
                }
            }
        }
        assert_eq!(wall_count, expected_wall_count, "wrong wall count");
        assert_eq!(floor_count, expected_floor_count, "wrong floor count");
    }

    #[test]
    fn add_items() {
        let mut map = create_map();
        let mut prefab = PrefabVault::new(PrefabBlueprint::Fortress);

        prefab.create_at(map.center, &mut map);

        let item_set: HashSet<char> = HashSet::from_iter("IPS".chars());
        let expected = PrefabBlueprint::Fortress
            .blueprint()
            .chars()
            .filter(|c| item_set.contains(c))
            .count();
        assert_eq!(map.item_positions.len(), expected, "wrong item count");
        assert!(
            map.item_positions
                .iter()
                .any(|i| i.item == Some(MagicItem::Map)),
            "Magic Map missing"
        );
        assert!(
            map.item_positions
                .iter()
                .any(|i| i.item == Some(MagicItem::Weapon(Weapon::HugeSword))),
            "Huge Sword missing"
        );
    }

    #[test]
    fn add_monsters() {
        let mut map = create_map();
        let mut prefab = PrefabVault::new(PrefabBlueprint::Fortress);

        prefab.create_at(map.center, &mut map);

        let monster_set: HashSet<char> = HashSet::from_iter("MO".chars());
        let expected = prefab
            .blueprint
            .blueprint()
            .chars()
            .filter(|c| monster_set.contains(c))
            .count();
        assert_eq!(
            map.monster_starting_positions.len(),
            expected,
            "wrong monster count"
        );
        assert!(
            map.monster_starting_positions
                .iter()
                .any(|m| m.monster == Some(Monster::Orc)),
            "Orc missing"
        );
    }

    #[test]
    fn location_within_dungeon() {
        let map = create_map();
        let prefab = PrefabVault::new(PrefabBlueprint::Fortress);

        let location = prefab
            .determine_location(&map, &mut RandomGenerator::new())
            .expect("no location found");

        let vault = prefab.vault_rect(location);
        let dungeon =
            IRect::from_center_size(IVec2::ZERO, IVec2::new(MAP_WIDTH as i32, MAP_HEIGHT as i32));
        assert!(
            dungeon.contains(vault.min),
            "vault min at {} is out of bounds",
            vault.min
        );
        assert!(
            dungeon.contains(vault.max),
            "vault max at {} is out of bounds",
            vault.max
        );
    }

    #[test]
    fn location_far_from_players() {
        let map = create_map();
        let prefab = PrefabVault::new(PrefabBlueprint::Fortress);

        let location = prefab
            .determine_location(&map, &mut RandomGenerator::new())
            .expect("no location found");

        let vault_zone = prefab.vault_rect(location).inflate(6);
        map.player_starting_positions
            .clone()
            .into_iter()
            .for_each(|pos| {
                assert!(
                    !vault_zone.contains(pos.into()),
                    "player at {} is within vault zone",
                    pos
                );
            });
    }

    #[test]
    fn off_dungeon_center() {
        let map = create_map();
        let prefab = PrefabVault::new(PrefabBlueprint::Fortress);

        let location = prefab
            .determine_location(&map, &mut RandomGenerator::new())
            .expect("no location found");

        let vault = prefab.vault_rect(location);
        assert!(
            !vault.contains(map.center.into()),
            "vault at {location} contains dungeon center"
        );
    }

    #[test]
    fn remove_pre_existing_monsters() {
        let mut map = create_map();
        let mut prefab = PrefabVault::new(PrefabBlueprint::Fortress);
        let location = prefab
            .determine_location(&map, &mut RandomGenerator::new())
            .expect("no location found");
        let vault = prefab.vault_rect(location);
        let monster_pos =
            MonsterPosition::new(DungeonPosition::from_vec2(vault.center().as_vec2()));
        map.monster_starting_positions.push(monster_pos);

        prefab.create_at(location, &mut map);

        assert!(
            map.monster_starting_positions
                .iter()
                .find(|&pos| pos.eq(&monster_pos))
                .is_none(),
            "monster at {monster_pos:?} was not removed"
        );
    }

    #[test]
    fn remove_pre_existing_items() {
        let mut map = create_map();
        let mut prefab = PrefabVault::new(PrefabBlueprint::Fortress);
        let location = prefab
            .determine_location(&map, &mut RandomGenerator::new())
            .expect("no location found");
        let vault = prefab.vault_rect(location);
        let item_pos = ItemPosition::new(DungeonPosition::from_vec2(vault.center().as_vec2()));
        map.item_positions.push(item_pos);

        prefab.create_at(location, &mut map);

        assert!(
            map.item_positions
                .iter()
                .find(|pos| item_pos.eq(&pos))
                .is_none(),
            "item at {item_pos:?} was not removed"
        );
    }

    #[test]
    fn ensure_reachable() {
        let mut map = create_map();
        let mut prefab = PrefabVault::new(PrefabBlueprint::Fortress);
        let location = prefab
            .determine_location(&map, &mut RandomGenerator::new())
            .expect("no location found");

        prefab.create_at(location, &mut map);

        map.player_starting_positions.iter().for_each(|&pos| {
            let path_finder = AStarPathFinder::find(pos, prefab.key_pos, &map);
            assert!(
                path_finder.path_found(),
                "player at {pos} is unable to reach key at {}",
                prefab.key_pos
            );
        });
    }

    fn create_map() -> DungeonMap {
        let mut map = DungeonMap::new();
        map.player_starting_positions
            .push(DungeonPosition::new(X_MIN + 1, Y_MIN + 1));
        map.player_starting_positions
            .push(DungeonPosition::new(X_MAX - 1, Y_MAX - 1));

        map
    }
}
