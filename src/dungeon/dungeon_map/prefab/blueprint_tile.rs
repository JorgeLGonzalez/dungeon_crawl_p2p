use super::*;
use crate::{
    items::{MagicItem, Weapon},
    monsters::Monster,
    prelude::*,
};

/// A blueprint tile specification for a dungeon prefab in a specified dungeon
/// position.
#[derive(Debug, Eq, PartialEq)]
pub enum BlueprintTile {
    Floor(DungeonPosition),
    Item(DungeonPosition),
    KeyMarker(DungeonPosition),
    Map(DungeonPosition),
    Monster(DungeonPosition),
    Ogre(DungeonPosition),
    Sword(DungeonPosition),
    Wall(DungeonPosition),
}

impl BlueprintTile {
    /// Create a dungeon tile with a monster or item as specified by the blueprint
    /// char.
    pub fn new(tile: char, pos: DungeonPosition) -> Self {
        match tile {
            '-' => BlueprintTile::Floor(pos),
            '#' => BlueprintTile::Wall(pos),
            'I' => BlueprintTile::Item(pos),
            'M' => BlueprintTile::Monster(pos),
            'O' => BlueprintTile::Ogre(pos),
            'S' => BlueprintTile::Sword(pos),
            'P' => BlueprintTile::Map(pos),
            'X' => BlueprintTile::KeyMarker(pos),
            _ => unreachable!("Unknown tile type: '{tile}'"),
        }
    }

    /// Add the tile and item or monster to the dungeon map.
    pub fn add_to(&self, map: &mut DungeonMap) {
        map.set_tile_type(&self.pos(), self.tile_type());
        match self {
            BlueprintTile::Floor(_) => {}
            BlueprintTile::Item(pos) => {
                map.add_one_item(ItemPosition::new(*pos));
            }
            BlueprintTile::KeyMarker(_) => {}
            BlueprintTile::Map(pos) => {
                map.add_one_item(ItemPosition::new_with_item(*pos, MagicItem::Map));
                trace!("Magic Map placed at {pos}");
            }
            BlueprintTile::Monster(pos) => {
                map.add_one_monster(MonsterPosition::new(*pos));
            }
            BlueprintTile::Ogre(pos) => {
                map.add_one_monster(MonsterPosition::new_with_monster(*pos, Monster::Orc));
            }
            BlueprintTile::Sword(pos) => {
                map.add_one_item(ItemPosition::new_with_item(
                    *pos,
                    MagicItem::Weapon(Weapon::HugeSword),
                ));
            }
            BlueprintTile::Wall(_) => {}
        }
    }

    pub fn pos(&self) -> DungeonPosition {
        match self {
            BlueprintTile::Floor(pos) => *pos,
            BlueprintTile::Item(pos) => *pos,
            BlueprintTile::KeyMarker(pos) => *pos,
            BlueprintTile::Map(pos) => *pos,
            BlueprintTile::Monster(pos) => *pos,
            BlueprintTile::Ogre(pos) => *pos,
            BlueprintTile::Sword(pos) => *pos,
            BlueprintTile::Wall(pos) => *pos,
        }
    }

    pub fn tile_type(&self) -> TileType {
        match self {
            BlueprintTile::Wall(_) => TileType::Wall,
            _ => TileType::Floor,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::floor('-', BlueprintTile::Floor(center()))]
    #[case::wall('#', BlueprintTile::Wall(center()))]
    #[case::item('I', BlueprintTile::Item(center()))]
    #[case::monster('M', BlueprintTile::Monster(center()))]
    #[case::ogre('O', BlueprintTile::Ogre(center()))]
    #[case::huge_sword('S', BlueprintTile::Sword(center()))]
    #[case::magic_map('P', BlueprintTile::Map(center()))]
    #[case::key_marker('X', BlueprintTile::KeyMarker(center()))]
    fn blueprint_tile(#[case] tile_char: char, #[case] expected: BlueprintTile) {
        assert_eq!(BlueprintTile::new(tile_char, expected.pos()), expected);
    }

    #[test]
    #[should_panic(expected = "Unknown tile type: ','")]
    fn invalid_tile_symbol() {
        BlueprintTile::new(',', center());
    }

    #[test]
    fn test_floor_tile() {
        let mut map = DungeonMap::new(1);
        let pos = center();

        BlueprintTile::Floor(pos).add_to(&mut map);

        assert_eq!(map.get_tile_type(&pos), TileType::Floor);
        assert!(map.item_positions().next().is_none());
        assert!(map.monster_starting_positions().next().is_none());
    }

    #[test]
    fn test_wall_tile() {
        let mut map = DungeonMap::new(1);
        let pos = center();

        BlueprintTile::Wall(pos).add_to(&mut map);

        assert_eq!(map.get_tile_type(&pos), TileType::Wall);
        assert!(map.item_positions().next().is_none());
        assert!(map.monster_starting_positions().next().is_none());
    }

    #[test]
    fn test_item_placement() {
        let mut map = DungeonMap::new(1);
        let pos = center();

        BlueprintTile::Item(pos).add_to(&mut map);

        assert_eq!(map.get_tile_type(&pos), TileType::Floor);
        assert_eq!(map.item_positions().count(), 1);
        assert_eq!(map.item_positions().nth(0).unwrap().pos, pos);
        assert_eq!(map.item_positions().nth(0).unwrap().item, None);
    }

    #[test]
    fn test_monster_placement() {
        let mut map = DungeonMap::new(1);
        let pos = center();

        BlueprintTile::Monster(pos).add_to(&mut map);

        assert_eq!(map.get_tile_type(&pos), TileType::Floor);
        assert_eq!(map.monster_starting_positions().count(), 1);
        assert_eq!(map.monster_starting_positions().nth(0).unwrap().pos, pos);
        assert_eq!(
            map.monster_starting_positions().nth(0).unwrap().monster,
            None
        );
    }

    #[test]
    fn test_ogre_placement() {
        let mut map = DungeonMap::new(1);
        let pos = center();

        BlueprintTile::Ogre(pos).add_to(&mut map);

        assert_eq!(map.get_tile_type(&pos), TileType::Floor);
        assert_eq!(map.monster_starting_positions().count(), 1);
        assert_eq!(map.monster_starting_positions().nth(0).unwrap().pos, pos);
        assert_eq!(
            map.monster_starting_positions().nth(0).unwrap().monster,
            Some(Monster::Orc)
        );
    }

    #[test]
    fn test_huge_sword_placement() {
        let mut map = DungeonMap::new(1);
        let pos = center();

        BlueprintTile::Sword(pos).add_to(&mut map);

        assert_eq!(map.get_tile_type(&pos), TileType::Floor);
        assert_eq!(map.item_positions().count(), 1);
        assert_eq!(map.item_positions().nth(0).unwrap().pos, pos);
        assert_eq!(
            map.item_positions().nth(0).unwrap().item,
            Some(MagicItem::Weapon(Weapon::HugeSword))
        );
    }

    #[test]
    fn test_map_placement() {
        let mut map = DungeonMap::new(1);
        let pos = center();

        BlueprintTile::Map(pos).add_to(&mut map);

        assert_eq!(map.get_tile_type(&pos), TileType::Floor);
        assert_eq!(map.item_positions().count(), 1);
        assert_eq!(map.item_positions().nth(0).unwrap().pos, pos);
        assert_eq!(
            map.item_positions().nth(0).unwrap().item,
            Some(MagicItem::Map)
        );
    }

    #[test]
    fn test_key_marker() {
        let mut map = DungeonMap::new(1);
        let pos = center();

        BlueprintTile::KeyMarker(pos).add_to(&mut map);

        assert_eq!(map.get_tile_type(&pos), TileType::Floor);
        assert!(map.item_positions().next().is_none());
        assert!(map.monster_starting_positions().next().is_none());
    }

    fn center() -> DungeonPosition {
        DungeonPosition::new(0, 0)
    }
}
