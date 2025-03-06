use super::*;
use crate::prelude::*;

/// Selects a random, but valid, location for the vault
pub(super) struct VaultSiteSelector {
    dimensions: IVec2,
    /// min distance around vault from player
    setback: i32,
    x_range_max: isize,
    x_range_min: isize,
    y_range_max: isize,
    y_range_min: isize,
}

impl VaultSiteSelector {
    pub fn new(dimensions: IVec2) -> Self {
        Self {
            dimensions,
            setback: 6,
            x_range_max: 0,
            x_range_min: 0,
            y_range_max: 0,
            y_range_min: 0,
        }
        .set_bounds()
    }

    /// Select a random, but valid location for the vault, if possible within
    /// a limited number of retries. The vault must be fully within the dungeon,
    /// far enough from players, and exclude the dungeon center.
    pub fn select(&self, map: &DungeonMap, rng: &mut RandomGenerator) -> Option<DungeonPosition> {
        let mut retries = 0;
        let mut location = None;

        while location.is_none() && retries < 10 {
            let pos = self.random_center_pos(rng);
            let vault = self.vault_rect(pos);
            let excludes_center = !vault.contains(map.center.into());

            if excludes_center && self.excludes_players(vault, map) {
                location = Some(pos);
            } else {
                retries += 1;
            }
        }

        location
    }

    fn excludes_players(&self, vault: IRect, map: &DungeonMap) -> bool {
        let vault_zone = vault.inflate(self.setback);

        !map.player_starting_positions
            .iter()
            .any(|&p| vault_zone.contains(p.into()))
    }

    fn random_center_pos(&self, rng: &mut RandomGenerator) -> DungeonPosition {
        let x = rng.gen_range(self.x_range_min..self.x_range_max);
        let y = rng.gen_range(self.y_range_min..self.y_range_max);

        DungeonPosition::new(x, y)
    }

    /// Set bounds for possible positions for the vault's center. The vault must
    /// be fully within the dungeon walls (so we add 1 to the min values, but
    /// need not subtract from max as those are already exclusive).
    fn set_bounds(mut self) -> Self {
        let half_width = self.dimensions.x as isize / 2;
        self.x_range_max = X_MAX - half_width;
        self.x_range_min = X_MIN + 1 + half_width;

        let half_height = self.dimensions.y as isize / 2;
        self.y_range_max = Y_MAX - half_height;
        self.y_range_min = Y_MIN + 1 + half_height;

        self
    }

    fn vault_rect(&self, pos: DungeonPosition) -> IRect {
        IRect::from_center_size(pos.into(), self.dimensions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn location_within_dungeon() {
        let map = create_map();
        let dimensions = IVec2::splat(10);
        let selector = VaultSiteSelector::new(dimensions);

        let pos = selector
            .select(&map, &mut RandomGenerator::new())
            .expect("No position found");

        let dungeon_rect =
            IRect::from_center_size(IVec2::ZERO, IVec2::new(MAP_WIDTH as i32, MAP_HEIGHT as i32));
        assert!(dungeon_rect.contains(pos.into()));
        assert!(dungeon_rect.contains(selector.vault_rect(pos).max));
        assert!(dungeon_rect.contains(selector.vault_rect(pos).min));
        assert!(!selector.vault_rect(pos).contains(map.center.into()));
        map.player_starting_positions.iter().for_each(|&p| {
            assert!(!selector
                .vault_rect(pos)
                .inflate(selector.setback)
                .contains(p.into()));
        });
    }

    #[test]
    fn excludes_players() {
        let mut map = create_map();
        // add 2 more players to center of top quadrants should block big vault placement
        let top_left = DungeonPosition::new(X_MIN / 2, Y_MAX / 2);
        map.player_starting_positions.push(top_left);
        let top_right = DungeonPosition::new(X_MAX / 2, Y_MAX / 2);
        map.player_starting_positions.push(top_right);
        let dimensions = IVec2::new(MAP_WIDTH as i32 / 2 - 5, MAP_HEIGHT as i32 - 3);
        let selector = VaultSiteSelector::new(dimensions);

        let pos = selector.select(&map, &mut RandomGenerator::new());

        assert!(pos.is_none());
    }

    #[test]
    fn fully_within_dungeon() {
        let mut map = create_map();
        map.player_starting_positions.clear();
        let dimensions = half_dungeon();
        let selector = VaultSiteSelector::new(dimensions);

        let pos = selector.select(&map, &mut RandomGenerator::new());

        assert!(pos.is_none(), "Position: {:?}", pos);
    }

    #[test]
    fn exclude_map_center() {
        let mut map = create_map();
        map.player_starting_positions.clear();
        let dimensions = max_dimensions();
        let selector = VaultSiteSelector::new(dimensions);

        let pos = selector.select(&map, &mut RandomGenerator::new());

        assert!(pos.is_none());
    }

    fn create_map() -> DungeonMap {
        let mut map = DungeonMap::new();
        map.player_starting_positions
            .push(DungeonPosition::new(X_MIN + 1, Y_MIN + 1));
        map.player_starting_positions
            .push(DungeonPosition::new(X_MAX - 1, Y_MAX - 1));

        map
    }

    /// Dimensions are ~half the dungeon horizontally and almost the full height
    /// to minimize vault placement options.
    fn half_dungeon() -> IVec2 {
        IVec2::new(MAP_WIDTH as i32 / 2, MAP_HEIGHT as i32 - 3)
    }

    fn max_dimensions() -> IVec2 {
        IVec2::new(MAP_WIDTH as i32 - 3, MAP_HEIGHT as i32 - 3)
    }
}
