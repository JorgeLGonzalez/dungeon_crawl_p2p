use super::DungeonPosition;
use crate::monsters::Monster;
use bevy::math::IVec2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MonsterPosition {
    pub monster: Option<Monster>,
    pub pos: DungeonPosition,
}

impl MonsterPosition {
    pub fn new(pos: DungeonPosition) -> Self {
        Self { pos, monster: None }
    }

    pub fn new_with_monster(pos: DungeonPosition, monster: Monster) -> Self {
        Self {
            pos,
            monster: Some(monster),
        }
    }
}

impl std::fmt::Display for MonsterPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at {}",
            self.monster.map_or("Monster".to_string(), |m| m.label()),
            self.pos
        )
    }
}

impl From<MonsterPosition> for DungeonPosition {
    fn from(monster_pos: MonsterPosition) -> Self {
        monster_pos.pos
    }
}

impl From<MonsterPosition> for IVec2 {
    fn from(monster_pos: MonsterPosition) -> Self {
        monster_pos.pos.into()
    }
}
