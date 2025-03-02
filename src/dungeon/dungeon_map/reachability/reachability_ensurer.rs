use super::*;
use bevy::log::info;
use std::fmt::Display;

/// Used to ensure that all searchers (e.g. players) can reach the target
pub struct ReachabilityEnsurer;

impl ReachabilityEnsurer {
    /// Ensures that all searchers (e.g. players) can reach the given target
    /// dungeon position. If no path exists, create a simple L-shaped tunnel
    /// between the closest forward and reverse path positions.
    pub fn ensure(searchers: &[Searchers], target: DungeonPosition, map: &mut DungeonMap) {
        searchers.iter().for_each(|searcher| {
            let finder = AStarPathFinder::find(searcher.pos, target, &map);
            if !finder.path_found() {
                let searcher_side = finder.closest_position();
                info!("Connecting {searcher} to pos {searcher_side}");
                let target_side =
                    AStarPathFinder::find(target, searcher_side, &map).closest_position();
                Tunneler::tunnel(map, searcher_side, target_side);
            }
        });
    }
}

/// A named dungeon denizen (e.g. player) at a given position. Used as an argument
/// to [`ReachabilityEnsurer::ensure`].
pub struct Searchers {
    name: String,
    pos: DungeonPosition,
}

impl Searchers {
    pub fn from_iter<I>(prefix: &str, positions: I) -> Vec<Self>
    where
        I: IntoIterator<Item = DungeonPosition>,
    {
        positions
            .into_iter()
            .enumerate()
            .map(|(i, pos)| Self::new(format!("{prefix} {i}"), pos))
            .collect()
    }

    pub fn new(name: String, pos: DungeonPosition) -> Self {
        Self { name, pos }
    }
}

impl Display for Searchers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Searcher {name} at {pos}",
            name = self.name,
            pos = self.pos
        )
    }
}
