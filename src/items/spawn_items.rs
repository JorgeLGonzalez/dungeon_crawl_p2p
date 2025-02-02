use super::{Grabbable, MagicItemTemplate};
use crate::{
    common::{DungeonAssets, DungeonData},
    hud::TooltipLabel,
    items::components::MagicItemBundle,
    prelude::*,
};
use bevy::utils::hashbrown::HashMap;
use bevy_ggrs::AddRollbackCommandExtension;
use std::iter::repeat;

pub fn spawn_items(
    dungeon: Res<DungeonMap>,
    dungeon_assets: Res<DungeonAssets>,
    dungeon_data_assets: Res<Assets<DungeonData>>,
    mut commands: Commands,
    mut rng: ResMut<RandomGenerator>,
) {
    let item_distribution = create_distribution(dungeon_data_assets.get(&dungeon_assets.data));

    let mut stats: HashMap<String, usize> = HashMap::new();

    for item_bundle in dungeon
        .item_positions
        .iter()
        .map(|pos| {
            (
                item_distribution[rng.gen_range(0..item_distribution.len())],
                pos.to_vec2(),
            )
        })
        .map(|(template, pos)| MagicItemBundle::new(template, pos))
    {
        stats
            .entry(item_bundle.item.label())
            .and_modify(|count| *count += 1)
            .or_insert(1);

        commands.spawn(item_bundle).add_rollback();
    }

    info!("Spawned items: {stats:?}");
}

/// Create a distribution of item templates based on their frequency so that
/// those with a higher frequency are more likely to be randomly selected.
fn create_distribution(dungeon_data: Option<&DungeonData>) -> Vec<&MagicItemTemplate> {
    dungeon_data
        .expect("Failed to load dungeon data")
        .items
        .iter()
        .flat_map(|template| repeat(template).take(template.frequency))
        .collect()
}
