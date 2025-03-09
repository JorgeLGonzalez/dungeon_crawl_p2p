use super::LevelText;
use crate::prelude::*;

pub fn update_level_ui(mut level_ui: Query<&mut Text, With<LevelText>>, map: Res<DungeonMap>) {
    level_ui.single_mut().0 = format!("Level {}", map.level);
}
