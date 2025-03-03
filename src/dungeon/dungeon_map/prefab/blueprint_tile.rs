use super::*;
use crate::{
    items::{MagicItem, Weapon},
    monsters::Monster,
    prelude::*,
};

/// A blueprint tile specification for a dungeon prefab in a specified dungeon
/// position.
#[derive(Eq, PartialEq)]
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
                map.item_positions.push(ItemPosition::new(*pos));
                trace!("Item placed at {pos}");
            }
            BlueprintTile::KeyMarker(_) => {}
            BlueprintTile::Map(pos) => {
                map.item_positions
                    .push(ItemPosition::new_with_item(*pos, MagicItem::Map));
                trace!("Magic Map placed at {pos}");
            }
            BlueprintTile::Monster(pos) => {
                map.monster_starting_positions
                    .push(MonsterPosition::new(*pos));
                trace!("Monster placed at {pos}");
            }
            BlueprintTile::Ogre(pos) => {
                map.monster_starting_positions
                    .push(MonsterPosition::new_with_monster(*pos, Monster::Orc));
                trace!("Orc placed at {pos}");
            }
            BlueprintTile::Sword(pos) => {
                map.item_positions.push(ItemPosition::new_with_item(
                    *pos,
                    MagicItem::Weapon(Weapon::HugeSword),
                ));
                trace!("Huge Sword placed at {pos}");
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
