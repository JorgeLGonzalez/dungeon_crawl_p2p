use super::*;
use crate::prelude::*;

pub struct PrefabVault {
    blueprint: String,
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
            placeholder,
        }
    }

    pub fn create_at(&self, center: DungeonPosition, map: &mut DungeonMap) {
        let width = self.placeholder.width() as isize;
        let vault = IRect::from_center_size(center.into(), self.placeholder.size());

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
            .for_each(|(pos, c)| {
                self.create_tile(c, pos, map);
            });
    }

    pub fn determine_location(
        &self,
        map: &DungeonMap,
        rng: &mut RandomGenerator,
    ) -> Option<DungeonPosition> {
        let dungeon = IRect::from_center_size(
            map.center.into(),
            IVec2::new(MAP_WIDTH as i32, MAP_HEIGHT as i32),
        );

        let mut location = None;
        let mut retries = 0;
        while location.is_none() && retries < 10 {
            let x = rng.gen_range(X_MIN..X_MAX - self.placeholder.width() as isize - 1);
            let y = rng.gen_range(Y_MIN..Y_MAX - self.placeholder.height() as isize - 1);
            let pos = DungeonPosition::new(x, y);
            let vault = IRect::from_center_size(pos.into(), self.placeholder.size());

            if dungeon.contains(vault.min) && dungeon.contains(vault.max) {
                location = Some(DungeonPosition::new(x, y));
            } else {
                retries += 1;
            }
        }

        location
    }

    fn create_tile(&self, c: char, pos: DungeonPosition, map: &mut DungeonMap) {
        match c {
            '-' => map.set_tile_type(&pos, TileType::Floor),
            '#' => map.set_tile_type(&pos, TileType::Wall),
            'M' => {
                map.set_tile_type(&pos, TileType::Floor);
                map.monster_starting_positions.push(pos);
            }
            _ => unreachable!(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::math::IVec2;

    #[test]
    fn new() {
        let prefab = PrefabVault::new(FORTRESS);

        assert_eq!(prefab.placeholder.width(), 12, "wrong width");
        assert_eq!(prefab.placeholder.height(), 11, "wrong height");
    }

    #[test]
    fn create_at() {
        let mut map = DungeonMap::new();
        let pos = map.center;
        let prefab = PrefabVault::new(FORTRESS);

        prefab.create_at(pos, &mut map);

        let expected_floor_count = FORTRESS.chars().filter(|c| *c == '-' || *c == 'M').count();
        let expected_wall_count = FORTRESS.chars().filter(|c| *c == '#').count();
        let expected_monster_count = FORTRESS.chars().filter(|c| *c == 'M').count();
        let vault = IRect::from_center_size(pos.into(), prefab.placeholder.size());

        assert_eq!(
            map.monster_starting_positions.len(),
            expected_monster_count,
            "wrong monster count"
        );
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
    fn determine_location() {
        let map = DungeonMap::new();
        let prefab = PrefabVault::new(FORTRESS);

        let location = prefab.determine_location(&map, &mut RandomGenerator::new());

        assert!(location.is_some());
        let location = location.unwrap();
        let vault = IRect::from_center_size(location.into(), prefab.placeholder.size());
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

    // TODO validate blueprint. allow blank lines at top and bottom, otherwise all must be same len
}
