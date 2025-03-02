use super::*;
use crate::prelude::*;

pub struct PrefabVault {
    blueprint: String,
    key_pos: DungeonPosition,
    placeholder: IRect,
}

impl PrefabVault {
    pub fn new(blueprint: &str) -> Self {
        let width = blueprint
            .chars()
            .skip(1)
            .position(|c| c == '\n' || c == '\r')
            .expect("No newline in blueprint") as i32;
        let height = (blueprint.lines().count() as i32) - 1;
        let placeholder = IRect::new(0, 0, width, height);

        Self {
            blueprint: blueprint.to_string(),
            key_pos: DungeonPosition::new(0, 0),
            placeholder,
        }
    }

    pub fn create_at(&mut self, center: DungeonPosition, map: &mut DungeonMap) {
        let width = self.placeholder.width() as isize;
        let vault = self.vault_rect(center);

        self.clear_content(vault, map);

        let to_pos = |idx: usize| -> DungeonPosition {
            let dx = idx as isize % width;
            let dy = idx as isize / width;

            DungeonPosition::new(vault.min.x as isize + dx, vault.min.y as isize + dy)
        };

        self.blueprint
            .chars()
            .filter(|c| *c != '\n' && *c != '\r')
            .enumerate()
            .map(|(idx, c)| (to_pos(idx), c))
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(pos, c)| {
                self.create_tile(c, pos, map);
            });

        info!("Vault created at {center}.");
    }

    pub fn determine_location(
        &self,
        map: &DungeonMap,
        rng: &mut RandomGenerator,
    ) -> Option<DungeonPosition> {
        let dungeon = self.dungeon_rect();

        let mut location = None;
        let mut retries = 0;
        while location.is_none() && retries < 10 {
            let x = rng.gen_range(X_MIN..X_MAX - self.placeholder.width() as isize - 1);
            let y = rng.gen_range(Y_MIN..Y_MAX - self.placeholder.height() as isize - 1);
            let pos = DungeonPosition::new(x, y);
            let vault = self.vault_rect(pos);

            let vault_zone = vault.inflate(6);
            let players_in_vault_zone = map
                .player_starting_positions
                .iter()
                .any(|&p| vault_zone.contains(p.into()));
            if !vault.contains(map.center.into())
                && !players_in_vault_zone
                && dungeon.contains(vault.min)
                && dungeon.contains(vault.max)
            {
                location = Some(pos);
            } else {
                retries += 1;
            }
        }

        location
    }

    fn clear_content(&self, vault: IRect, map: &mut DungeonMap) {
        map.item_positions
            .retain(|&pos| !vault.contains(pos.into()));
        map.monster_starting_positions
            .retain(|&pos| !vault.contains(pos.into()));
    }

    fn create_tile(&mut self, c: char, pos: DungeonPosition, map: &mut DungeonMap) {
        match c {
            '-' => map.set_tile_type(&pos, TileType::Floor),
            '#' => map.set_tile_type(&pos, TileType::Wall),
            'I' => {
                map.set_tile_type(&pos, TileType::Floor);
                map.item_positions.push(pos);
            }
            'M' => {
                map.set_tile_type(&pos, TileType::Floor);
                map.monster_starting_positions.push(pos);
            }
            'X' => {
                map.set_tile_type(&pos, TileType::Floor);
                self.key_pos = pos;
            }
            _ => unreachable!(),
        };
    }

    fn dungeon_rect(&self) -> IRect {
        IRect::from_center_size(IVec2::ZERO, IVec2::new(MAP_WIDTH as i32, MAP_HEIGHT as i32))
    }

    fn vault_rect(&self, pos: DungeonPosition) -> IRect {
        IRect::from_center_size(pos.into(), self.placeholder.size())
    }
}

#[cfg(test)]
mod tests {
    use super::{reachability::AStarPathFinder, *};

    #[test]
    fn new() {
        let prefab = PrefabVault::new(FORTRESS);

        assert_eq!(prefab.placeholder.width(), 12, "wrong width");
        assert_eq!(prefab.placeholder.height(), 11, "wrong height");
    }

    #[test]
    fn create_at() {
        let mut map = create_map();
        let pos = map.center;
        let mut prefab = PrefabVault::new(FORTRESS);

        prefab.create_at(pos, &mut map);

        let blueprint = FORTRESS
            .chars()
            .filter(|c| *c != '\n' && *c != '\r')
            .collect::<String>();
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
        assert_eq!(floor_count, expected_floor_count, "wrong floor count");
        assert_eq!(wall_count, expected_wall_count, "wrong wall count");
    }

    #[test]
    fn add_items() {
        let mut map = create_map();
        let mut prefab = PrefabVault::new(FORTRESS);

        prefab.create_at(map.center, &mut map);

        let expected = FORTRESS.chars().filter(|c| *c == 'I').count();
        assert_eq!(map.item_positions.len(), expected, "wrong item count");
    }

    #[test]
    fn add_monsters() {
        let mut map = create_map();
        let mut prefab = PrefabVault::new(FORTRESS);

        prefab.create_at(map.center, &mut map);

        let expected = FORTRESS.chars().filter(|c| *c == 'M').count();
        assert_eq!(
            map.monster_starting_positions.len(),
            expected,
            "wrong monster count"
        );
    }

    #[test]
    fn location_within_dungeon() {
        let map = create_map();
        let prefab = PrefabVault::new(FORTRESS);

        let location = prefab
            .determine_location(&map, &mut RandomGenerator::new())
            .expect("no location found");

        let vault = prefab.vault_rect(location);
        let dungeon = prefab.dungeon_rect();
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
        let prefab = PrefabVault::new(FORTRESS);

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
        let prefab = PrefabVault::new(FORTRESS);

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
        let mut prefab = PrefabVault::new(FORTRESS);
        let location = prefab
            .determine_location(&map, &mut RandomGenerator::new())
            .expect("no location found");
        let vault = prefab.vault_rect(location);
        let monster_pos = DungeonPosition::from_vec2(vault.center().as_vec2());
        map.monster_starting_positions.push(monster_pos);

        prefab.create_at(location, &mut map);

        assert!(
            map.monster_starting_positions
                .iter()
                .find(|&pos| pos.eq(&monster_pos))
                .is_none(),
            "monster at {monster_pos} was not removed"
        );
    }

    #[test]
    fn remove_pre_existing_items() {
        let mut map = create_map();
        let mut prefab = PrefabVault::new(FORTRESS);
        let location = prefab
            .determine_location(&map, &mut RandomGenerator::new())
            .expect("no location found");
        let vault = prefab.vault_rect(location);
        let item_pos = DungeonPosition::from_vec2(vault.center().as_vec2());
        map.item_positions.push(item_pos);

        prefab.create_at(location, &mut map);

        assert!(
            map.item_positions
                .iter()
                .find(|&pos| pos.eq(&item_pos))
                .is_none(),
            "item at {item_pos} was not removed"
        );
    }

    #[test]
    fn ensure_reachable() {
        let mut map = create_map();
        let mut prefab = PrefabVault::new(FORTRESS);
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
