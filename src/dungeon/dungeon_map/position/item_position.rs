use super::DungeonPosition;
use crate::items::MagicItem;
use bevy::math::IVec2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ItemPosition {
    pub pos: DungeonPosition,
    pub item: Option<MagicItem>,
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
