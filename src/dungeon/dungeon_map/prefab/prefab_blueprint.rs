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

    pub fn dimensions(&self) -> IVec2 {
        let blueprint = self.blueprint();
        let width = blueprint
            .chars()
            .skip(1) // first line is blank
            .position(|c| c == '\n' || c == '\r')
            .expect("No newline in blueprint") as i32;
        let height = (blueprint.lines().count() as i32) - 1;

        IVec2::new(width, height)
    }

    pub fn tiles(&self, vault: IRect) -> impl Iterator<Item = BlueprintTile> + use<'_> {
        let width = vault.width() as isize;

        let to_pos = move |idx: usize| -> DungeonPosition {
            let dx = idx as isize % width;
            let dy = idx as isize / width;
            // note y-axis is inverted
            DungeonPosition::new(vault.min.x as isize + dx, vault.max.y as isize - dy)
        };

        self.blueprint()
            .chars()
            .filter(|c| *c != '\n' && *c != '\r')
            .enumerate()
            .map(move |(idx, c)| BlueprintTile::new(c, to_pos(idx)))
    }
}

#[cfg(test)]
mod tests {
    use super::blueprints::FORTRESS;
    use super::*;
    use rstest::rstest;

    #[test]
    fn rect() {
        let rect = PrefabBlueprint::Fortress.dimensions();

        assert_eq!(rect, IVec2::new(12, 11));
    }

    #[test]
    fn tiles() {
        let vault = IRect::new(10, 10, 22, 21);

        let tiles = PrefabBlueprint::Fortress.tiles(vault).collect::<Vec<_>>();

        assert_eq!(tiles.len() as i32, vault.width() * vault.height());
        let mut index = 0;
        for x in 10..=22 {
            for y in 21..=10 {
                let pos = DungeonPosition::new(x as isize, y as isize);
                assert_eq!(tiles[index].pos(), pos);
                index += 1;
            }
        }
    }

    #[rstest]
    #[case::fortress(PrefabBlueprint::Fortress, FORTRESS)]
    fn test_blueprint(#[case] blueprint: PrefabBlueprint, #[case] raw_blueprint: &str) {
        assert_eq!(blueprint.blueprint(), raw_blueprint);

        let expected_len = raw_blueprint.lines().skip(1).next().unwrap().len();
        for (row_number, row) in raw_blueprint.lines().skip(1).enumerate() {
            assert_eq!(
                row.len(),
                expected_len,
                "invalid row length for row {row_number}"
            );
        }
        let dimensions = blueprint.dimensions();
        let vault = IRect::from_center_size(IVec2::ZERO, dimensions);
        let tiles = blueprint.tiles(vault).collect::<Vec<_>>();
        assert_eq!(tiles.len() as i32, dimensions.x * dimensions.y);
        let x_pos = tiles
            .iter()
            .find(|t| matches!(t, BlueprintTile::KeyMarker(_)));
        assert!(x_pos.is_some(), "missing X marker");
    }
}
