use super::*;
use crate::dungeon::{DungeonMap, DungeonPosition};
use bevy::math::IRect;

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
        let x0 = center.x - width / 2;
        let y0 = center.y - self.placeholder.height() as isize / 2;

        self.blueprint
            .chars()
            .filter(|c| *c != '\n' && *c != '\r')
            .enumerate()
            .map(|(idx, c)| ((idx as isize % width, idx as isize / width), c))
            .map(|((dx, dy), c)| (DungeonPosition::new(x0 + dx, y0 + dy), c))
            .for_each(|(pos, c)| {
                match c {
                    '-' => map.set_tile_type(&pos, TileType::Floor),
                    '#' => map.set_tile_type(&pos, TileType::Wall),
                    'M' => {
                        map.set_tile_type(&pos, TileType::Floor);
                        map.monster_starting_positions.push(pos);
                    }
                    _ => unreachable!(),
                };
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            // let mut row = String::new();
            for x in vault.min.x..vault.max.x {
                let tile = map.get_tile_type(&DungeonPosition::new(x as isize, y as isize));
                match tile {
                    TileType::Floor => floor_count += 1,
                    TileType::Wall => wall_count += 1,
                    _ => unreachable!(),
                }

                // match tile {
                //     TileType::Floor => row.push('-'),
                //     TileType::Wall => row.push('#'),
                //     _ => unreachable!(),
                // }
            }
            // println!("{row}");
        }
        assert_eq!(floor_count, expected_floor_count, "wrong floor count");
        assert_eq!(wall_count, expected_wall_count, "wrong wall count");
    }

    // TODO validate blueprint. allow blank lines at top and bottom, otherwise all must be same len
}
