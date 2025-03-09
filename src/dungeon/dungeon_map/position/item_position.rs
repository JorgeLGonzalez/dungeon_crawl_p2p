use super::DungeonPosition;
use crate::items::MagicItem;
use bevy::math::IVec2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ItemPosition {
    pub item: Option<MagicItem>,
    pub pos: DungeonPosition,
}

impl ItemPosition {
    pub fn new(pos: DungeonPosition) -> Self {
        Self { pos, item: None }
    }

    pub fn new_with_item(pos: DungeonPosition, item: MagicItem) -> Self {
        Self {
            pos,
            item: Some(item),
        }
    }
}

impl std::fmt::Display for ItemPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at {}",
            self.item.map_or("Item".to_string(), |i| i.label()),
            self.pos
        )
    }
}

impl From<ItemPosition> for DungeonPosition {
    fn from(item_pos: ItemPosition) -> Self {
        item_pos.pos
    }
}

impl From<ItemPosition> for IVec2 {
    fn from(item_pos: ItemPosition) -> Self {
        item_pos.pos.into()
    }
}
