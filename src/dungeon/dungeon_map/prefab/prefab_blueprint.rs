use super::{blueprints, *};
use crate::prelude::*;

#[derive(Debug)]
pub enum PrefabBlueprint {
    Fortress,
}

impl PrefabBlueprint {
    pub fn blueprint(&self) -> &'static str {
        match self {
            PrefabBlueprint::Fortress => &blueprints::FORTRESS,
        }
    }

    pub fn rect(&self) -> IRect {
        let blueprint = self.blueprint();
        let width = blueprint
            .chars()
            .skip(1) // first line is blank
            .position(|c| c == '\n' || c == '\r')
            .expect("No newline in blueprint") as i32;
        let height = (blueprint.lines().count() as i32) - 1;

        IRect::new(0, 0, width, height)
    }

    pub fn tiles(&self, vault: IRect) -> Vec<BlueprintTile> {
        let width = vault.width() as isize;

        let to_pos = |idx: usize| -> DungeonPosition {
            let dx = idx as isize % width;
            let dy = idx as isize / width;
            // note y-axis is inverted
            DungeonPosition::new(vault.min.x as isize + dx, vault.max.y as isize - dy)
        };

        self.blueprint()
            .chars()
            .filter(|c| *c != '\n' && *c != '\r')
            .enumerate()
            .map(|(idx, c)| BlueprintTile::new(c, to_pos(idx)))
            .collect()
    }
}
